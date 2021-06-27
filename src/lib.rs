

#[allow(dead_code)]

pub mod ecs;

use std::iter::FromIterator;
use ecs::{ Entity, Component };

#[derive(Debug)]
pub enum SwarmError {
    MemoryLayoutFailure,
}

type Pointer = usize;
pub type Spawn = usize;

pub struct Swarm<T: Default + Copy> {
    entities: Box<[Entity]>,
    map: Box<[Pointer]>,
    content: Vec<T>,
    free: Vec<Pointer>,
    len: usize,
    max: usize,
}

impl<T: Default + Copy> Swarm<T> {

    pub fn new(size: usize) -> Self {
        Swarm { 
            entities: Box::from_iter((0..size).into_iter().map(|_i| Entity (0))),
            map: Box::from_iter((0..size).into_iter()),
            content: vec![T::default(); size],
            free: Vec::new(),
            len: 0,
            max: size,
        }
    }

    pub fn spawn(&mut self) -> Option<Spawn> {
        if self.len < self.max {
            self.len += 1;
            if self.free.len() > 0 {
                self.free.pop()
            } else {
                Some(self.map[self.len-1])
            }
        } else {
            None
        }
    }


    // The content at index map[id] is swapped with the last entity
    // value of map[id] is changed to point to the last entity
    // value of map[last pointer] is changed to id
    // len is shortened by one

    pub fn kill(&mut self, target: Spawn) {
        self.entities[self.map[target]].clear();
        
        if self.len > 1 {
            let last_pos = self.len - 2;
            let target_pos = self.map[target];
            // swap content to back
            self.content[target_pos] = self.content[last_pos];  
            self.entities[target_pos] = self.entities[last_pos];
            // swap content pointers in map
            self.map[target] = self.map[last_pos];
            self.map[last_pos] = target_pos;

            self.free.push(target);
        }
        // decrement size             
        self.len -= 1;                                      
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

    pub fn add_component(&mut self, id: &Spawn, component: Component) {
        self.entities[self.map[*id]].add_component(component);
    }

    pub fn remove_component(&mut self, id: &Spawn, component: Component) {
        self.entities[self.map[*id]].remove_component(component);
    }

    pub fn count(&self) -> usize { self.len }

    pub fn max_size(&self) -> usize { self.max }
}


