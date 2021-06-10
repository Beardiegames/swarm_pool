#[allow(dead_code)]

use std::iter::FromIterator;
use std::collections::VecDeque;
mod tests;

#[derive(Debug)]
pub enum SwarmError {
    MemoryLayoutFailure,
}

type Pointer = usize;
pub type SpawnId = usize;

pub trait System {
    type Entity;
    fn update(&mut self, entity: &mut Self::Entity);
}


pub struct Swarm<T: Default + Copy> {
    map: Box<[Pointer]>,
    content: Vec<T>,
    free: Vec<Pointer>,
    len: usize,
    max: usize,
    system: Box<dyn System<Entity = T>>,
}

impl<T: Default + Copy> Swarm<T> {

    pub fn new(size: usize, system: Box<dyn System<Entity = T>>) -> Self {
        Swarm { 
            map: Box::from_iter((0..size).into_iter()),
            content: vec![T::default(); size],
            free: Vec::new(),
            len: 0,
            max: size,
            system,
        }
    }

    pub fn spawn(&mut self) -> Option<SpawnId> {
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

    // What the hell happens here?
    //
    //          1xA          3xC           2+A
    // id:  1A|2B|3C  >  1x|2B|3C  >  1x|2B|3x  >  1x|2B|3C
    //      --------     --------     --------     --------
    // map: 1 |2 |3   >  3 |2 | 1  >  3 | 1| 2  >  2 |1 | 3  // swap with pointer last
    // con: A |B |C   >  C |B |*A  >  B |*C|*A  >  B |C |*A  // swap with last

    pub fn kill(&mut self, id: SpawnId) {
        if self.len > 1 {
            // swap content to back
            let id_ptr = self.map[id];
            self.content[id_ptr] = self.content[self.len-2];  
            // swap content pointers in map
            self.map[id] = self.map[self.len-2];
            self.map[self.len-2] = id_ptr;

            self.free.push(id);
        }
        // decrement size             
        self.len -= 1;                                      
    }

    pub fn for_each(&mut self) {
        for i in 0..self.len {
            self.system.update(&mut self.content[i]);
        }
    }

    pub fn get_mut(&mut self, id: Pointer) -> &mut T {
        &mut self.content[id]
    }
}
