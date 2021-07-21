//! Types used by the Swarm pool.

use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

use super::control::SwarmControl;

/// A pointer to a swarm data object
pub type ObjectPosition = usize;

/// The identity of a Spawn
pub type SpawnId = usize;

/// A callback handler used by the for_each() methode on Swarm.
/// Return a mutable reference of a data object in the pool that 
/// the for_each() loop is currently iterating over.
pub type ForEachHandler<ItemType> = fn(&mut ItemType);

/// A callback handler used by the for_all() methode on Swarm.
/// Returns the object position, a mutable pool reference and the swarm properties  
/// the for_all() loop is currently iterating over.
pub type ForAllHandler<ItemType, Properties> = fn(&ObjectPosition, &mut [ItemType], &mut Properties);

/// A callback handler used by the update() methode on Swarm.
/// Return a SwarmControl object that refers to the object the update() loop 
/// is currently iterating over.
pub type UpdateHandler<ItemType, Properties> = fn(&mut SwarmControl<ItemType, Properties>);

// spawns and tags

/// A spawn is a pointer that points to a data object in the swarm pool.
/// Spawns are 'Reference Counted' which makes it possible to hand them out 
/// like free candy during halloween, no (compiler) questions asked ;)
pub struct Spawn(pub(crate) Rc<RefCell<Tag>>);

impl Spawn {
    pub(crate) fn new(index: usize) -> Self {
        Spawn( Rc::new( RefCell::new( Tag{ id:index, pos:index, active:false })))
    }

    /// Returns the identity of this Spawns. All RC clones of this spawn have the same 'id' 
    /// and point to the same object position.
    pub fn id(&self) -> SpawnId { 
        self.0.borrow().id 
    }

    /// Returns the position of the target object in the pool
    pub fn pos(&self) -> ObjectPosition { 
        self.0.borrow().pos 
    }

    /// Returns true if this object is active and will be updated any of the loop methodes
    pub fn active(&self) -> bool { 
        self.0.borrow().active 
    }

    /// Returns a 'Reference Counted' clone of this Spawn
    pub fn mirror(&self) -> Self { 
        Spawn (Rc::clone(&self.0)) 
    }
}

impl fmt::Debug for Spawn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

/// Spawns are compared by their identity or 'id' value
impl PartialEq for Spawn {
    fn eq(&self, other: &Spawn) -> bool {
        self.id() == other.id()
    }
}

/// Tags hold Spawn data, and since A spawn is a Refence Counted Tag, that makes a Tag kind of an abstract Spawn
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