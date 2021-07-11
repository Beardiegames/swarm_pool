#[allow(dead_code)]
mod tests;
pub mod error;
pub mod control;

use std::rc::Rc;
use std::cell::RefCell;
use control::SwarmControl;

// types

// pub trait SwarmType: Default + Copy {}
// pub trait SwarmProperties {}

pub type Spawn = Rc<RefCell<Tag>>;
pub type ObjectIndex = usize;
pub type SpawnId = usize;

pub type ForEachHandler<T> = fn(&mut T);
pub type ForAllHandler<T, P> = fn(&ObjectIndex, &mut [T], &mut P);
pub type UpdateHandler<T, P> = fn(&SpawnId, &mut Swarm<T, P>);

// tags

#[derive(Default, Debug)]
pub struct Tag {
    pub(crate) id: SpawnId,
    pub(crate) pos: ObjectIndex,
    pub(crate) active: bool,
}

#[allow(dead_code)]
impl Tag {
    fn id(&self) -> &SpawnId { &self.id }
    fn pos(&self) -> &ObjectIndex { &self.pos }
    fn active(&self) -> bool { self.active } 
}

// swarm code

pub struct Swarm<T: Default + Copy, P> {
    pool: Vec<T>,
    tags: Vec<Spawn>,
    control: SwarmControl,
    pub properties: P,
}

impl<T: Default + Copy, P> Swarm<T, P> {

    pub fn new(size: usize, properties: P) -> Self {
        let mut tags = Vec::<Rc<RefCell<Tag>>>::new();
        for i in 0..size { 
            let tag = Rc::new(RefCell::new(Tag{ id:i, pos:i, active:false }));
            tags.push(tag);
        }

        Swarm { 
            pool: vec![T::default(); size],
            tags,
            control: SwarmControl::new(size),
            properties,
        }
    }

    // pooling

    pub fn spawn(&mut self) -> Option<Rc<RefCell<Tag>>> { // Result<SpawnId, error::SwarmError> {
        match self.control.spawn() { 
            Some((id, index)) => {
                let tag = &self.tags[index];
                    tag.borrow_mut().id = id;
                    tag.borrow_mut().pos = index;
                    tag.borrow_mut().active = true;

                Some(Rc::clone(&tag))
            },
            None => None
        }
    }

    pub fn kill(&mut self, target: Spawn) -> Result<(), error::SwarmError> {
        let pool = &mut self.pool;
        let tags = &mut self.tags;

        self.control.kill(target.borrow().id, |from, to| {
            pool[*to] = pool[*from];

            let to_tag = tags[*to].clone();
            let from_tag = tags[*from].clone();
            tags[*to] = from_tag;
            tags[*from] = to_tag;
        })                              
    }

    pub fn delayed_kill(&mut self, target: SpawnId) {
        self.control.delayed_kill(target);
    }

    // states

    // pub fn is_active(&self, tag: &Spawn) -> bool {
    //     self.control.is_active(&tag.id)
    // }

    pub fn get_mut(&mut self, spawn: &Spawn) -> &mut T { 
        &mut self.pool[self.control.map[spawn.borrow().id]]
    }

    pub fn get_ref(&self, spawn: &Spawn) -> &T { 
        &self.pool[self.control.map[spawn.borrow().id]]
    }

    pub fn raw_mut(&mut self, spawn_id: &SpawnId) -> &mut T { 
        &mut self.pool[self.control.map[*spawn_id]]
    }

    pub fn raw_ref(&self, spawn_id: &SpawnId) -> &T { 
        &self.pool[self.control.map[*spawn_id]]
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

    pub fn for_all(&mut self, handler: ForAllHandler<T, P>) {
        let len = self.control.len;
        let mut i = 0;

        while i < len {
            handler(&i, &mut self.pool, &mut self.properties);
            i += 1;
        }
    }

    pub fn update(&mut self, handler: UpdateHandler<T, P>) {
        
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

        #[allow(unused_must_use)]
        while i < len2 {
            self.kill(
                Rc::clone(
                    &self.tags[
                        self.control.map[
                            self.control.delayed_kills[i]
                        ]
                    ]
                )
            );
            i += 1;
        }
        
        self.control.dkill_amount = 0;
    }
}





