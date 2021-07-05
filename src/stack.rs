use super::*;
use error::*;
use std::iter::FromIterator;
use std::convert::TryInto;
use std::fmt::Debug;

pub fn update<T>(swarm: &mut Swarm<T>, handler: fn(&Spawn, &mut Swarm<T>)) 
where   T: Default + Clone + Debug
{
    swarm.is_updateing = true;
    for i in 0..swarm.len.clone() {
        handler(&i, swarm);
    }

    swarm.is_updateing = false;
    for i in 0..swarm.dkill_amount {
        swarm.kill(swarm.delayed_kills[i]);
    }
}


pub struct Swarm<T: Default + Clone + Debug> {
    map: Box<[Pointer]>,
    content: [T; 1_000],
    free: Vec<Pointer>,
    len: usize,
    max: usize,
    is_updateing: bool,
    delayed_kills: Box<[Spawn]>,
    dkill_amount: usize,
}

impl<T: Default + Clone + Debug> Swarm<T> {

    pub fn new() -> Self {
        Swarm { 
            map: Box::from_iter((0..1_000).into_iter()),
            content: vec![T::default(); 1_000].try_into()
                .expect("Failed to create swarm due to spawn type!"),
            free: Vec::new(),
            len: 0,
            max: 1_000,
            is_updateing: false,
            delayed_kills: Box::from_iter((0..1_000).into_iter()),
            dkill_amount: 0,
        }
    }

    // The content at index map[id] is swapped with the last entity
    // value of map[id] is changed to point to the last entity
    // value of map[last pointer] is changed to id
    // len is shortened by one

    pub fn kill(&mut self, target: Spawn) -> Result<(), SwarmError> {
        if self.is_updateing { 
            return Err(error::KILL_DURING_UPDATE) 
        }

        if self.len > 1 {
            let last_pos = self.len - 2;
            let target_pos = self.map[target];
            // swap content to back
            self.content[target_pos] = self.content[last_pos].clone();  
            // swap content pointers in map
            self.map[target] = self.map[last_pos];
            self.map[last_pos] = target_pos;

            self.free.push(target);
        }
        // decrement size             
        if self.len > 0 { 
            self.len -= 1; 
        } else {

        }   
        Ok(())                                  
    }

    pub fn delayed_kill(&mut self, spawn: Spawn) {
        let mut allready_in_que = false;
        for i in 0..self.dkill_amount {
            if self.delayed_kills[i] == spawn {
                allready_in_que = true;
            }
        }
        if !allready_in_que {
            self.delayed_kills[self.dkill_amount] = spawn;
            self.dkill_amount += 1;
        }
    }

    pub fn spawn(&mut self) -> Result<Spawn, SwarmError> {
        if self.len < self.max {
            self.len += 1;
            if self.free.len() > 0 {
                self.free.pop().ok_or(error::UNKNOWN)
            } else {
                Ok(self.map[self.len-1])
            }
        } else {
            Err(error::MAX_SPAWNS_REACHED)
        }
    }

    pub fn for_each(&mut self, handler: fn(&mut T)) {
        for i in 0..self.len {
            handler(&mut self.content[i]);
        }
    }

    pub fn is_active(&self, id: &Spawn) -> bool {
        self.map[*id] < self.len
    }

    pub fn get_mut(&mut self, id: &Spawn) -> &mut T { 
        &mut self.content[self.map[*id]] 
    }

    pub fn get_ref(&self, id: &Spawn) -> &T { 
        &self.content[self.map[*id]] 
    }

    pub fn count(&self) -> usize { self.len }

    pub fn max_size(&self) -> usize { self.max }
}