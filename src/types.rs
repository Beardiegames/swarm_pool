use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

use super::context::SwarmContext;

// basic types

pub type ObjectPosition = usize;
pub type SpawnId = usize;

// update handlers

pub type ForEachHandler<T> = fn(&mut T);
pub type ForAllHandler<T, P> = fn(&ObjectPosition, &mut [T], &mut P);
pub type UpdateHandler<T, P> = fn(&mut SwarmContext<T, P>);

// spawns and tags

pub struct Spawn(pub(crate) Rc<RefCell<Tag>>);

impl Spawn {
    pub(crate) fn new(index: usize) -> Self {
        Spawn( Rc::new( RefCell::new( Tag{ id:index, pos:index, active:false })))
    }

    pub fn id(&self) -> SpawnId { 
        self.0.borrow().id 
    }

    pub fn pos(&self) -> ObjectPosition { 
        self.0.borrow().pos 
    }

    pub fn active(&self) -> bool { 
        self.0.borrow().active 
    }

    pub fn mirror(&self) -> Self { 
        Spawn (Rc::clone(&self.0)) 
    }
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
    fn id(&self) -> &SpawnId            { &self.id }
    fn pos(&self) -> &ObjectPosition    { &self.pos }
    fn active(&self) -> bool            { self.active } 
}