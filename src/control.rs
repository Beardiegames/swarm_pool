
use std::iter::FromIterator;
use super::*;

pub struct SwarmControl {
    free: Vec<Spawn>,
    //pub(crate) map: Box<[ObjectIndex]>,
    pub(crate) len: usize,
    pub(crate) max: usize,
    pub(crate) is_updateing: bool,
    pub(crate) delayed_kills: Vec<SpawnId>,
    pub(crate) dkill_amount: usize,
}

impl SwarmControl {

    pub fn new(size: usize) -> Self {
        SwarmControl { 
            //map: Box::from_iter((0..size).into_iter()),
            free: Vec::with_capacity(size),
            len: 0,
            max: size,
            is_updateing: false,
            delayed_kills: vec![0; size],
            dkill_amount: 0,
        }
    }

    // The content at index map[id] is swapped with the last entity
    // value of map[id] is changed to point to the last entity
    // value of map[last pointer] is changed to id
    // len is shortened by one

    pub fn kill<F>(&mut self, target: &Spawn, copy_handler: F) 
        -> Result<(), error::SwarmError> 
    where F: FnOnce(&usize, &usize)
    {
        if self.is_updateing { 
            return Err(error::KILL_DURING_UPDATE) 
        }

        if self.len > 1 {
            let last_ptr = self.len - 2;
            let target_ptr = target.0.borrow().pos; //self.map[target];

            // swap content to back
            // self.pool[target_ptr] = self.pool[last_ptr].clone(); 
            copy_handler(&last_ptr, &target_ptr);

            // swap content pointers in map
            // self.map[target] = last_ptr;
            // self.map[last_ptr] = target_ptr;

            self.free.push(target.mirror());
        }
        // decrement size             
        if self.len > 0 { 
            self.len -= 1; 
        } else {

        }   
        Ok(())                                  
    }

    pub fn delayed_kill(&mut self, spawn: SpawnId) {
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

    pub fn spawn(&mut self) -> Option<Spawn> { //(SpawnId, ObjectIndex)> {
        if self.len < self.max {
            self.len += 1;
            let pos = self.len-1;

            if self.free.len() > 0 {
                self.free.pop().map(|s| { s.0.borrow_mut().pos = pos; s }) // .ok_or(error::UNKNOWN)
            } else {
                
                let tag = &self.tags[index];
                    tag.0.borrow_mut().id = pos;
                    tag.0.borrow_mut().pos = pos;
                    tag.0.borrow_mut().active = true;

                Some((id, self.map[id]))
            }
        } else {
            //Err(error::MAX_SPAWNS_REACHED)
            None
        }
    }

    // pub fn is_active(&self, id: &SpawnId) -> bool {
    //     self.map[*id] < self.len
    // }

    pub fn count(&self) -> usize { self.len }

    pub fn max_size(&self) -> usize { self.max }
}