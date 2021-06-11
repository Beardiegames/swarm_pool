
#[allow(dead_code)]

use std::iter::FromIterator;
mod tests;

#[derive(Debug)]
pub enum SwarmError {
    MemoryLayoutFailure,
}

type Pointer = usize;
pub type Spawn = usize;
pub type Component = usize;

#[derive(Copy, Clone)]
pub struct Entity(u32);

impl Entity {
    pub fn clear(&mut self) { self.0 = 0; }

    pub fn add_component(&mut self, component: Component) {
        self.0 |= 1 << component;
    }

    pub fn remove_component(&mut self, component: Component) {
        self.0 &= !(1 << component);
    }
}

pub struct System<T: Default + Copy> {
    req: Entity,
    updater: fn(&mut T),
}

impl<T:Default + Copy> System<T> {

    fn new(require_components: &[Component], updater: fn(&mut T)) -> Self {
        let mut req = Entity (0);
        for c in require_components {
            req.add_component(*c);
        }
        System { req, updater }
    }

    fn run(&mut self, swarm: &mut Swarm<T>) {
        for i in 0..swarm.len {
            if self.req.0 == swarm.entities[i].0 & self.req.0 {
                (self.updater)(&mut swarm.content[i]);
            }
        }
    }
}

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
            self.entities[self.len-1].add_component(0);

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

    pub fn get_mut(&mut self, id: Pointer) -> &mut T {
        &mut self.content[id]
    }
}
