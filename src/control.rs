//! Controling the swarm inside its own swarm.update() loop
//! 
//! Example
//! ```
//!     extern crate swarm_pool;
//!     use swarm_pool::{ Swarm, Spawn };
//!     
//!     // setup a custom swarm properties object use for data sharing
//!     pub struct FollowSpawns {
//!         john: Option<Spawn>,
//!         cristy: Option<Spawn>,
//!     }
//!
//!     // setup a custom pool data object
//!     #[derive(Default, Clone)]
//!     pub struct Minion {
//!         name: &'static str,
//!         value: usize,
//!     }
//!
//!     // create a new swarm using the custom data and property types
//!     let mut swarm = Swarm::<Minion, FollowSpawns>::new(10, FollowSpawns {
//!         john: None,
//!         cristy: None,
//!     });
//!     
//!     // create two new spawns
//!     let s_john = swarm.spawn().unwrap();
//!     let s_cristy = swarm.spawn().unwrap();
//!
//!     // duplicate the spawn references into the swarm properties object
//!     swarm.properties.john = Some(s_john.mirror());
//!     swarm.properties.cristy = Some(s_cristy.mirror());
//!
//!     // set the name fields
//!     swarm.fetch(&s_john).name = "John";
//!     swarm.fetch(&s_cristy).name = "Cristy";
//!
//!     // loop over all spawns
//!     swarm.update(|ctl| {
//!         let name = ctl.target().name;
//!     
//!         // get spawn references from the swarm properties
//!         let cristy = ctl.properties.cristy.as_ref().unwrap().mirror();
//!         let john = ctl.properties.john.as_ref().unwrap().mirror();
//!         
//!         // if the currently updating pool object is john
//!         if name == "John" { 
//!             // john tells critsy to have a value of 2
//!             ctl.fetch(&cristy).value = 2; 
//!         }
//!         // if the currently updating pool object is Cristy
//!         if name == "Cristy" { 
//!             // cristy tells john to have a value of 1
//!             ctl.fetch(&john).value = 1; 
//!         }
//!     });
//!
//!     assert_eq!(swarm.fetch_ref(&s_john).value, 1);
//!     assert_eq!(swarm.fetch_ref(&s_cristy).value, 2);
//! ```

use super::types::*;

/// SwarmControl is passed as a parameter to the UpdateHandler during the 
/// swarm.update() loop. The Swarm Control object holds references to swarm pooling
/// values, this makes it possible to make changes to the swarm pool inside the 
/// update loop, without having to move Swarm out of itself. 
pub struct SwarmControl<'a, ItemType, Properties> {
    pub(crate) max: &'a usize,
    pub(crate) spawns: &'a mut Vec<Spawn>,
    pub(crate) free: &'a mut Vec<Spawn>,
    pub(crate) order: &'a mut Vec<usize>,

    pub(crate) len: usize,
    pub(crate) pos: usize, // the pool index of the currently updating spawn
    
    pub pool: &'a mut Vec<ItemType>,
    pub properties: &'a mut Properties,
}

impl<'a, ItemType, Properties> SwarmControl<'a, ItemType, Properties> 
where ItemType: Default + Clone {

    /// Returns a mutable reference to the pool object that is currently being updated
    pub fn target(&mut self) -> &mut ItemType {
        &mut self.pool[self.pos]
    }

    /// Returns a Spawn that is linked to the current pool object being updated
    pub fn target_spawn(&self) -> Spawn {
        self.spawns[self.pos].mirror()
    }

    /// Returns the ObjectPosition, or pool index, where the currently updating pool
    /// object is located at this moment (pool position can change over time).
    pub fn head(&self) -> ObjectPosition {
        self.order[self.pos]
    }

    /// Returns a spawn reference object from an object position within the pool
    pub fn fetch_spawn(&self, pos: &ObjectPosition) -> Spawn {
        self.spawns[*pos].mirror()
    }

    /// Returns a mutable reference to an object from the Swarm pool.
    /// The supplied Spawn reference object points out which object to return.
    pub fn fetch(&mut self, spawn: &Spawn) -> &mut ItemType { 
        &mut self.pool[spawn.0.borrow().pos]
    }

    /// Returns a immutable reference to an object from the Swarm pool.
    /// The supplied Spawn reference object points out which object to return.
    pub fn fetch_ref(&self, spawn: &Spawn) -> &ItemType { 
        &self.pool[spawn.0.borrow().pos]
    }

    /// Returns the number of spawned instances currently availeble
    pub fn count(&self) -> usize { self.len }

    /// Returns the maximum number of instances that can be spawned
    pub fn capacity(&self) -> usize { self.max.clone() }

    /// Loop through spawned instances until the `predicate` callback returns true.
    /// 
    /// If the loop was interupted by the predicate it returns a Spawn that refers
    /// that pool object.
    /// If on all iterations of the loop the `predicate` was false, None will be returned. 
    ///
    /// # Example
    /// ```
    ///     extern crate swarm_pool;
    ///     use swarm_pool::{ Swarm, Spawn };
    /// 
    ///     let mut swarm = Swarm::<u8, _>::new(10, ());
    ///     swarm.populate(&[5, 4, 3, 2, 1]);
    /// 
    ///     assert_eq!(swarm.count(), 5);
    ///     
    ///     swarm.update(|ctl| {
    ///         if let Some(spawn) = ctl.find(|p| *p == 2) {
    ///             assert_eq!(spawn.pos(), 3);
    ///             assert_eq!(*ctl.fetch(&spawn), 2);
    ///         } else {
    ///             panic!("Spawn not found!");
    ///         }
    ///     });
    ///```
    pub fn find<Predicate> (&self, predicate: Predicate) -> Option<Spawn> 
    where Predicate: Fn(&ItemType) -> bool {
        let count = self.len;
        let mut i = 0;

        while &i < &count {
            if predicate(&self.pool[self.order[i]]) { 
                return Some(self.spawns[i].mirror());
            }
            i += 1;
        }
        return None
    }

    /// Loop through spawned instances until the `predicate` callback returns false.
    /// 
    /// If the loop was interupted by a false predicate it returns a Spawn that refers
    /// that pool object. If on all iterations of the loop the `predicate` was true, 
    /// None will be returned.
    /// 
    /// This methode functions in the opposite way as the find methode.
    pub fn for_while<Predicate> (&self, predicate: Predicate) -> Option<Spawn> 
    where Predicate: Fn(&ItemType) -> bool {
        let count = self.len;
        let mut i = 0;

        while &i < &count {
            if predicate(&self.pool[self.order[i]]) { 
                return Some( self.spawns[i].mirror());
            }
            i += 1;
        }
        return None
    }

    /// Loops through all spawned instances and returns them via a callback
    /// handler. The callback handler is supplied with a mutable reference of these
    /// instances so that the object data of each looped instance can be changed.
    ///
    /// This methode functions the same as the Swarm.enumerate() methode.
    pub fn enumerate(&mut self, handler: EnumerateHandler<ItemType>) {
        let len = self.len;
        let mut i = 0;

        while &i < &len {
            handler(&i, &mut self.pool[i]);
            i += 1;
        }
    }


    /// Loop through all spawned instances and edit them.
    /// 
    /// This methode functions the same as the Swarm.for_each() methode.
    pub fn for_each(&mut self, handler: ForEachHandler<ItemType>) {
        let count = self.len;
        let mut i = 0;

        while &i < &count {
            handler(&mut self.pool[self.order[i]]);
            i += 1;
        }
    }

    /// Loops through all spawned instances and returns their object position via a 
    /// callback handler
    /// 
    /// This methode functions the same as the Swarm.for_all() methode.
    pub fn for_all(&mut self, handler: ForAllHandler<ItemType, Properties>) {
        let len = self.len;
        let mut i = 0;

        while &i < &len {
            handler(&i, &mut self.pool, &mut self.properties);
            i += 1;
        }
    }

    /// Create a new pool instance an returns a linked Spawn reference.
    /// Spawns are pool instances that will be included in the update loops 
    /// provided by Swarm, as long as they are active and not killed yet.
    /// 
    /// Returns None if the pool reached it's maximum capacity and 
    /// therefore could not spawn new instances
    /// 
    /// **NOTE**: Spawns created by SwarmControl will be included the next time 
    /// Swarm.update() is called.
    /// 
    /// # Example
    /// ```
    ///     extern crate swarm_pool;
    ///     use swarm_pool::{ Swarm, Spawn };
    /// 
    ///     let mut swarm = Swarm::<u8, _>::new(10, ());
    ///     let spawn = &swarm.spawn().unwrap();
    ///     assert_eq!(swarm.count(), 1);
    ///     
    ///     swarm.update(|ctl| {
    ///         if ctl.head() == 0 { ctl.spawn();} 
    ///     });
    ///     assert_eq!(swarm.count(), 2);
    ///```
    pub fn spawn(&mut self) -> Option<Spawn> {
        if self.len < *self.max {

            self.len += 1;
            let pos = self.len - 1;
     
            if self.free.len() > 0 {
                self.free.pop().map(|s| { 
                    s.0.borrow_mut().pos = pos; 
                    s.0.borrow_mut().active = true;
                    s 
                })
            } else {
                
                let s = &self.spawns[pos];
                    s.0.borrow_mut().pos = pos;
                    s.0.borrow_mut().active = true;
    
                Some(s.mirror())
            }
        } else {
            None
        }
    }

    /// Remove the currently updating spawn instance, see SwarmControl.kill()
    pub fn kill_current(&mut self) {
        self.kill(&self.target_spawn())
    }

    /// Remove a spawn instance from the swarm pool update loops
    /// 
    /// **NOTE**: Spawns killed by SwarmControl will be excluded the next time 
    /// Swarm.update() is called.
    /// 
    /// # Example
    /// ```
    ///     extern crate swarm_pool;
    ///     use swarm_pool::{ Swarm, Spawn };
    /// 
    ///     let mut swarm = Swarm::<u8, _>::new(10, ());
    ///     let spawn = &swarm.spawn().unwrap();
    ///     assert_eq!(swarm.count(), 1);
    ///         
    ///     swarm.update(|ctl| {
    ///         let spawn = ctl.target_spawn();
    ///         if ctl.head() == 0 { ctl.kill(&spawn); }
    ///     });
    ///     assert_eq!(swarm.count(), 0);
    ///```
    pub fn kill(&mut self, target: &Spawn) {
        target.0.borrow_mut().active = false;
    
        let last_pos = self.len - 1;
        let target_pos = target.pos();
    
        if self.len > 1 && target_pos < last_pos {
            
            let last_val = self.pool[last_pos].clone();
            let target_val = self.pool[target_pos].clone();
    
            // swap content to back
            self.pool[target_pos] = last_val;
            self.pool[last_pos] = target_val;
    
            // swap spawns equally
            self.spawns[target_pos] = self.spawns[last_pos].mirror();
            self.spawns[last_pos] = target.mirror();
    
            // set swapped spawn pool pointer to point to their new location
            self.spawns[target_pos].0.borrow_mut().pos = target_pos;
            self.spawns[last_pos].0.borrow_mut().pos = last_pos;
    
            if target_pos > self.pos { self.order[target_pos] = last_pos; }
            if last_pos > self.pos { self.order[last_pos] = target_pos; }
        }
    
        // store and decrement size             
        if self.len > 0 { 
            self.free.push(target.mirror());
            self.len -= 1; 
        }
    }
}