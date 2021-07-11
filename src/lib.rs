#[allow(dead_code)]
mod tests;
pub mod error;
//pub mod control;

use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
//use control::SwarmControl;

// types

pub type ObjectPosition = usize;
pub type SpawnId = usize;

pub type ForEachHandler<T> = fn(&mut T);
pub type ForAllHandler<T, P> = fn(&ObjectPosition, &mut [T], &mut P);
pub type UpdateHandler<T, P> = fn(ObjectPosition, &mut Swarm<T, P>);

// spawns and tags

pub struct Spawn(Rc<RefCell<Tag>>);

impl Spawn {
    pub fn id(&self) -> SpawnId { self.0.borrow().id }
    pub fn pos(&self) -> ObjectPosition { self.0.borrow().pos }
    pub fn active(&self) -> bool { self.0.borrow().active }
    pub fn mirror(&self) -> Self { Spawn (Rc::clone(&self.0)) }
}

impl fmt::Debug for Spawn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl PartialEq for Spawn {
    fn eq(&self, other: &Spawn) -> bool {
        self.id() == other.id()
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Tag {
    pub(crate) id: SpawnId,
    pub(crate) pos: ObjectPosition,
    pub(crate) active: bool,
}

#[allow(dead_code)]
impl Tag {
    fn id(&self) -> &SpawnId { &self.id }
    fn pos(&self) -> &ObjectPosition { &self.pos }
    fn active(&self) -> bool { self.active } 
}

// swarm code

pub struct Swarm<T: Default + Copy, P> {
    pool: Vec<T>,
    spawns: Vec<Spawn>,
    free: Vec<Spawn>,
    len: usize,
    max: usize,
    iter: usize,

    pub properties: P,
}

impl<T: Default + Copy, P> Swarm<T, P> {

    pub fn new(size: usize, properties: P) -> Self {
        let mut spawns = Vec::<Spawn>::new();

        for i in 0..size { 
            let tag = Spawn (
                Rc::new(RefCell::new(Tag{ id:i, pos:i, active:false }))
            );
            spawns.push(tag);
        }

        Swarm { 
            pool: vec![T::default(); size],
            spawns,
            free: Vec::<Spawn>::with_capacity(size),
            len: 0,
            max: size,
            iter: 0,

            properties,
        }
    }

    // pooling

    pub fn spawn(&mut self) -> Option<Spawn> {
        if self.len < self.max {

            if self.iter >= self.len { self.iter += 1; }
            self.len += 1;
            let pos = self.len-1;

            if self.free.len() > 0 {
                self.free.pop().map(|s| { s.0.borrow_mut().pos = pos; s })
            } else {
                
                let tag = &self.spawns[pos];
                    tag.0.borrow_mut().id = pos;
                    tag.0.borrow_mut().pos = pos;
                    tag.0.borrow_mut().active = true;

                Some(tag.mirror())
            }
        } else {
            None
        }
    }

    pub fn kill(&mut self, target: &Spawn) {
        target.0.borrow_mut().active = false;
        
        let pool = &mut self.pool;
        let spawns = &mut self.spawns;

        if self.len > 1 {
            let last_pos = self.len - 1;
            let target_pos = target.0.borrow().pos;

            // swap content to back
            pool[target_pos] = pool[last_pos];
            spawns[target_pos].0.borrow_mut().pos = last_pos;
            spawns[last_pos].0.borrow_mut().pos = target_pos;

            self.free.push(target.mirror());

            // if in iter loop: 
            // if a not yet updated spawn was swapped with an already updated spawn 
            // the update iter head will be set to that spawns position
            if self.iter < last_pos {
                if target_pos == self.iter { 
                    self.iter -= 1; 
                }
                else if target_pos < self.iter { 
                    // swap swapped with current being updated, then subtract
                    let iter_pos = self.iter - 1;
                    let swapped = pool[target_pos];
                    let current = pool[iter_pos];

                    pool[target_pos] = current;
                    pool[iter_pos] = swapped;

                    spawns[target_pos].0.borrow_mut().pos = iter_pos;
                    spawns[iter_pos].0.borrow_mut().pos = target_pos;
                    self.iter -= 1;
                }
            }
        }

        // decrement size             
        if self.len > 0 { 
            self.len -= 1; 
        }                           
    }

    // pub fn delayed_kill(&mut self, target: SpawnId) {
    //     self.control.delayed_kill(target);
    // }

    // states & references

    pub fn get_spawn(&self, pos: &ObjectPosition) -> Spawn {
        self.spawns[*pos].mirror()
    }

    pub fn get_mut(&mut self, spawn: &Spawn) -> &mut T { 
        &mut self.pool[spawn.pos()]
    }

    pub fn get_ref(&self, spawn: &Spawn) -> &T { 
        &self.pool[spawn.pos()]
    }

    pub fn get_raw(&mut self, pos: &ObjectPosition) -> &mut T { 
        &mut self.pool[*pos]
    }

    pub fn count(&self) -> usize { self.len }

    pub fn max_size(&self) -> usize { self.max }

    // iterators

    pub fn for_each(&mut self, handler: fn(&mut T)) {
        let len = self.len;
        let mut i = 0;

        while &i < &len {
            handler(&mut self.pool[i]);
            i += 1;
        }
    }

    pub fn for_all(&mut self, handler: ForAllHandler<T, P>) {
        let len = self.len;
        let mut i = 0;

        while &i < &len {
            handler(&i, &mut self.pool, &mut self.properties);
            i += 1;
        }
    }
}

pub fn update<T, P> (swarm: &mut Swarm<T, P>, handler: UpdateHandler<T, P>) 
where T: Default + Copy
{
    //swarm.locked = true;

    //let len1 = swarm.len;
    //let len2 = swarm.control.dkill_amount;
    swarm.iter = 0;
    while &swarm.iter < &swarm.len {
        handler(swarm.iter, swarm);
        swarm.iter += 1;
    }

    //swarm.locked = false;

    //i = 0;

    // #[allow(unused_must_use)]
    // while i < len2 {
    //     self.kill(
    //         &self.spawns[
    //             self.control.map[
    //                 self.control.delayed_kills[i]
    //             ]
    //         ].mirror()
    //     );
    //     i += 1;
    // }
    
    // self.control.dkill_amount = 0;
}