#[allow(dead_code)]
mod tests;
pub mod error;
// pub mod heap;
// pub mod stack;

use std::iter::FromIterator;
use std::iter::Iterator;
use std::slice::IterMut;
use std::convert::TryInto;
use std::fmt::Debug;

type Pointer = usize;
pub type Spawn = usize;


// pub fn update<T>(swarm: &mut Swarm<T>, handler: fn(&Spawn, &mut Swarm<T>)) 
// where   T: Default + Clone + Debug
// {
//     swarm.control.is_updateing = true;
//     for i in 0..swarm.len.clone() {
//         handler(&i, swarm);
//     }

//     swarm.control.is_updateing = false;
//     for i in 0..swarm.dkill_amount {
//         swarm.kill(swarm.delayed_kills[i]);
//     }
// }

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

// pub enum Pool<T: Default + Clone + Debug> {
//     Stack64([T; 64]),
//     Stack128([T; 128]),
//     Stack256([T; 256]),
//     Stack512([T; 512]),
//     Stack1024([T; 1024]),

//     Heap(Box<[T]>),
// }

// impl<T: Default + Clone + Debug> Pool<T> {

//     pub fn new_heap(size: usize) -> Self {
//         Self::Heap(Box::from_iter(
//             vec![T::default(); size].into_iter()
//         ))
//     }

//     pub fn new_stack(size: StackSize) -> Self {
//         match size {
//             StackSize::Stack64 => Self::Stack64(vec![T::default(); 64]
//                 .try_into()
//                 .expect("Failed to create swarm due to spawn type!")),
//             StackSize::Stack128 => Self::Stack128(vec![T::default(); 128]
//                 .try_into()
//                 .expect("Failed to create swarm due to spawn type!")),
//             StackSize::Stack256 => Self::Stack256(vec![T::default(); 256]
//                 .try_into()
//                 .expect("Failed to create swarm due to spawn type!")),
//             StackSize::Stack512 => Self::Stack512(vec![T::default(); 512]
//                 .try_into()
//                 .expect("Failed to create swarm due to spawn type!")),
//             StackSize::Stack1024 => Self::Stack1024(vec![T::default(); 1024]
//                 .try_into()
//                 .expect("Failed to create swarm due to spawn type!")),
//         }
//     }

//     pub fn size(&self) -> usize {
//         match self {
//             Self::Stack64(pool) => 64,
//             Self::Stack128(pool) => 128,
//             Self::Stack256(pool) => 256,
//             Self::Stack512(pool) => 512,
//             Self::Stack1024(pool) => 1024,
//             Self::Heap(pool) => pool.len(),
//         }
//     }

//     pub fn copy(&mut self, from_index: &usize, to_index: &usize) {
//         match self {
//             Self::Stack64(pool) => pool[*to_index] = pool[*from_index].clone(),
//             Self::Stack128(pool) => pool[*to_index] = pool[*from_index].clone(),
//             Self::Stack256(pool) => pool[*to_index] = pool[*from_index].clone(),
//             Self::Stack512(pool) => pool[*to_index] = pool[*from_index].clone(),
//             Self::Stack1024(pool) => pool[*to_index] = pool[*from_index].clone(),
//             Self::Heap(pool) => pool[*to_index] = pool[*from_index].clone(),
//         }
//     }

//     pub fn for_each(&mut self, handler: fn(&mut T), count: &usize) {
//         match self {
//             Self::Stack64(pool) => for i in 0..*count { handler(&mut pool[i]) },
//             Self::Stack128(pool) => for i in 0..*count { handler(&mut pool[i]) },
//             Self::Stack256(pool) => for i in 0..*count { handler(&mut pool[i]) },
//             Self::Stack512(pool) => for i in 0..*count { handler(&mut pool[i]) },
//             Self::Stack1024(pool) => for i in 0..*count { handler(&mut pool[i]) },
//             Self::Heap(pool) => for i in 0..*count { handler(&mut pool[i]) },
//         }
//     }

//     pub fn get_mut(&mut self, at_index: &usize) -> &mut T {
//         match self {
//             Self::Stack64(pool) => &mut pool[*at_index],
//             Self::Stack128(pool) => &mut pool[*at_index],
//             Self::Stack256(pool) => &mut pool[*at_index],
//             Self::Stack512(pool) => &mut pool[*at_index],
//             Self::Stack1024(pool) => &mut pool[*at_index],
//             Self::Heap(pool) => &mut pool[*at_index],
//         }
//     }

//     pub fn get_ref(&self, at_index: &usize) -> &T {
//         match self {
//             Self::Stack64(pool) => &pool[*at_index],
//             Self::Stack128(pool) => &pool[*at_index],
//             Self::Stack256(pool) => &pool[*at_index],
//             Self::Stack512(pool) => &pool[*at_index],
//             Self::Stack1024(pool) => &pool[*at_index],
//             Self::Heap(pool) => &pool[*at_index],
//         }
//     }
// }

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
            let last_pos = self.len - 2;
            let target_pos = self.map[target];
            // swap content to back
                // self.pool[target_pos] = self.pool[last_pos].clone(); 
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


// pub trait Swarm<T: Default + Copy> {
//     fn spawn(&mut self) -> Result<Spawn, error::SwarmError>;
//     fn kill(&mut self, target: Spawn) -> Result<(), error::SwarmError>;
//     fn delayed_kill(&mut self, spawn: Spawn);
    
//     fn is_active(&self, id: &Spawn) -> bool;
//     fn get_mut(&mut self, id: &Spawn) -> &mut T;
//     fn get_ref(&self, id: &Spawn) -> &T;
//     fn count(&self) -> usize;
//     fn max_size(&self) -> usize;

//     fn for_each(&mut self, handler: fn(&mut T));
//     fn update(&mut self, handler: fn(&Spawn, &mut Self));
// }

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

//}

// impl<T: Default + Copy> Swarm<T> for StackSwarm<T> {

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

    pub fn for_each(&mut self, handler: fn(&mut T)) {
        for i in 0..self.control.len {
            handler(&mut self.pool[i]);
        }
    }

    pub fn update(&mut self, handler: fn(&Spawn, &mut Self)) {
        self.control.is_updateing = true;
        for i in 0..self.control.len {
            handler(&i, self);
        }

        self.control.is_updateing = false;
        for i in 0..self.control.dkill_amount {
            self.kill(self.control.delayed_kills[i]);
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

// }

// impl<T: Default + Copy> Swarm<T> for HeapSwarm<T> {

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

    pub fn for_each(&mut self, handler: fn(&mut T)) {
        for i in 0..self.control.len {
            handler(&mut self.pool[i]);
        }
    }

    pub fn update(&mut self, handler: fn(&Spawn, &mut Self)) {
        self.control.is_updateing = true;
        for i in 0..self.control.len {
            handler(&i, self);
        }

        self.control.is_updateing = false;
        for i in 0..self.control.dkill_amount {
            self.kill(self.control.delayed_kills[i]);
        }
        self.control.dkill_amount = 0;
    }
}





