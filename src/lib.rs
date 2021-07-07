#[allow(dead_code)]
mod tests;
pub mod error;

use std::iter::FromIterator;

type Pointer = usize;
pub type Spawn = usize;


pub enum PoolSize {
    Stack(StackSize),
    Heap(usize),
}

pub enum StackSize {
    Stack64 = 64,
    Stack128 = 128,
    Stack256 = 256,
    Stack512 = 512,
    Stack1024 = 1024,
}

pub struct SwarmControl {
    map: Box<[Pointer]>,
    free: Vec<Pointer>,
    len: usize,
    max: usize,
    is_updateing: bool,
    delayed_kills: Vec<Spawn>,
    dkill_amount: usize,
}

impl SwarmControl {

    pub fn new(size: usize) -> Self {
        SwarmControl { 
            map: Box::from_iter((0..size).into_iter()),
            free: Vec::new(),
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

    pub fn kill<F>(&mut self, target: Spawn, copy_handler: F) 
        -> Result<(), error::SwarmError> 
    where F: FnOnce(&usize, &usize)
    {
        if self.is_updateing { 
            return Err(error::KILL_DURING_UPDATE) 
        }

        if self.len > 1 {
            let last_ptr = self.len - 2;
            let target_ptr = self.map[target];

            // swap content to back
            // self.pool[target_ptr] = self.pool[last_ptr].clone(); 
            copy_handler(&last_ptr, &target_ptr);

            // swap content pointers in map
            self.map[target] = last_ptr;
            self.map[last_ptr] = target_ptr;

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

    pub fn spawn(&mut self) -> Result<Spawn, error::SwarmError> {
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

    pub fn is_active(&self, id: &Spawn) -> bool {
        self.map[*id] < self.len
    }

    pub fn count(&self) -> usize { self.len }

    pub fn max_size(&self) -> usize { self.max }
}


pub struct StackSwarm<T: Default + Copy> {
    pool: [T; 1024],
    control: SwarmControl,
}

impl<T: Default + Copy> StackSwarm<T> {

    pub fn new() -> Self {
        StackSwarm { 
            pool: [T::default(); 1024],
            control: SwarmControl::new(1024),
        }
    }

    // ooling

    pub fn spawn(&mut self) -> Result<Spawn, error::SwarmError> {
        self.control.spawn()
    }

    pub fn kill(&mut self, target: Spawn) -> Result<(), error::SwarmError> {
        let pool = &mut self.pool;
        self.control.kill(target, |from, to| pool[*to] = pool[*from])                              
    }

    pub fn delayed_kill(&mut self, spawn: Spawn) {
        self.control.delayed_kill(spawn);
    }

    // states
    
    pub fn is_active(&self, id: &Spawn) -> bool {
        self.control.is_active(id)
    }

    pub fn get_mut(&mut self, id: &Spawn) -> &mut T { 
        &mut self.pool[self.control.map[*id]]
    }

    pub fn get_ref(&self, id: &Spawn) -> &T { 
        &self.pool[self.control.map[*id]]
    }

    pub fn count(&self) -> usize { self.control.len }
    pub fn max_size(&self) -> usize { self.control.max }

    // updaters

    pub fn for_each(&mut self, handler: fn(&mut T)) {
        let len = self.control.len;
        let mut i = 0;

        while i < len {
            handler(&mut self.pool[i]);
            i += 1;
        }
    }

    pub fn update(&mut self, handler: fn(&Pointer, &mut [T; 1024])) {
        let len = self.control.len;
        let mut i = 0;

        while i < len {
            handler(&i, &mut self.pool);
            i += 1;
        }
    }

    pub fn update_ctl(&mut self, handler: fn(&Spawn, &mut Self)) {
        self.control.is_updateing = true;

        let len1 = self.control.len;
        let len2 = self.control.dkill_amount;
        let mut i = 0;

        while i < len1 {
            handler(&i, self);
            i += 1;
        }

        self.control.is_updateing = false;

        i = 0;
        while i < len2 {
            self.kill(self.control.delayed_kills[i]);
            i += 1;
        }

        self.control.dkill_amount = 0;
    }
}

pub struct HeapSwarm<T: Default + Copy> {
    pool: Vec<T>,
    control: SwarmControl,
}

impl<T: Default + Copy> HeapSwarm<T> {

    pub fn new(size: usize) -> Self {
        HeapSwarm { 
            pool: vec![T::default(); size],
            control: SwarmControl::new(size),
        }
    }

    // pooling

    pub fn spawn(&mut self) -> Result<Spawn, error::SwarmError> {
        self.control.spawn()
    }

    pub fn kill(&mut self, target: Spawn) -> Result<(), error::SwarmError> {
        let pool = &mut self.pool;
        self.control.kill(target, |from, to| pool[*to] = pool[*from])                              
    }

    pub fn delayed_kill(&mut self, spawn: Spawn) {
        self.control.delayed_kill(spawn);
    }

    // states
    
    pub fn is_active(&self, id: &Spawn) -> bool {
        self.control.is_active(id)
    }

    pub fn get_mut(&mut self, id: &Spawn) -> &mut T { 
        &mut self.pool[self.control.map[*id]]
    }

    pub fn get_ref(&self, id: &Spawn) -> &T { 
        &self.pool[self.control.map[*id]]
    }

    pub fn count(&self) -> usize { self.control.len }
    pub fn max_size(&self) -> usize { self.control.max }

    // updaters

    pub fn for_each(&mut self, handler: fn(&mut T)) {
        let len = self.control.len;
        let mut i = 0;

        while i < len {
            handler(&mut self.pool[i]);
            i += 1;
        }
    }

    pub fn update(&mut self, handler: fn(&Pointer, &mut Vec<T>)) {
        let len = self.control.len;
        let mut i = 0;

        while i < len {
            handler(&i, &mut self.pool);
            i += 1;
        }
    }

    pub fn update_ctl(&mut self, handler: fn(&Spawn, &mut Self)) {
        
        self.control.is_updateing = true;

        let len1 = self.control.len;
        let len2 = self.control.dkill_amount;
        let mut i = 0;

        while i < len1 {
            handler(&i, self);
            i += 1;
        }

        self.control.is_updateing = false;

        i = 0;
        while i < len2 {
            self.kill(self.control.delayed_kills[i]);
            i += 1;
        }
        
        self.control.dkill_amount = 0;
    }
}





