

// #[macro_export]
// macro_rules! require {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut v = 0u32;
//             $(
//                 swarm::add_component!(v, $x);
//             )*
//             v
//         }
//     };
// }

// #[macro_export]
// macro_rules! add_component {
//     ($v:expr, $c:expr) => { $v = $v | 1 << $c; };
// }

#[allow(dead_code)]

pub mod ecs;

use std::iter::FromIterator;
use ecs::{ Entity };

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

    // What the hell happens here?
    //  -content[] is an array of the actual entity data objects
    //  -map[] is an array of pointers to the entities inside content
    //  -id's are the index pointers to map
    //
    // The content at index map[id] is swapped with the last entity
    // value of map[id] is changed to point to the last entity
    // value of map[last pointer] is changed to id
    // len is shortened by one
    //
    //          1xA          3xC           2+A
    // id:  1A|2B|3C  >  1x|2B|3C  >  1x|2B|3x  >  1x|2B|3C
    //      --------     --------     --------     --------
    // map: 1 |2 |3   >  3 |2 | 1  >  3 | 1| 2  >  2 |1 | 3  // swap with pointer last
    // con: A |B |C   >  C |B |*A  >  B |*C|*A  >  B |C |*A  // swap with last

    pub fn kill(&mut self, id: Spawn) {
        self.entities[self.map[id]].clear();
        
        if self.len > 1 {
            let last = self.len - 2;
            // swap content to back
            let id_ptr = self.map[id];
            self.content[id_ptr] = self.content[last];  
            self.entities[id_ptr] = self.entities[last];
            // swap content pointers in map
            self.map[id] = self.map[last];
            self.map[last] = id_ptr;

            self.free.push(id);
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

    pub fn add_component(&mut self, id: &Spawn, component: u8) {
        self.entities[self.map[*id]].add_component(component);
    }

    pub fn remove_component(&mut self, id: &Spawn, component: u8) {
        self.entities[self.map[*id]].remove_component(component);
    }

    pub fn count(&self) -> usize { self.len }

    pub fn max_size(&self) -> usize { self.max }
}


