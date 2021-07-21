//! An object pooling system optimized for perfomance.
//! 
//! The pooling system manages object instances of a cutom type,
//! and provides update loops to over them.
//! 
//! # Examples
//! ```
//! extern crate swarm;
//! use swarm::Swarm;
//! use swarm::tools::byte_str::ByteStr;
//! 
//! // create an object you want to pool
//! #[derive(Default, Copy, Clone)]     // The object you want to pool must implement these
//! pub struct MyPoolObject {           // Swarm uses Copy and therfore only accepts Sized properties!
//!     pub name: ByteStr,              // This means types such as String and Vec aren't allowed
//!     pub value: usize,               // The tools module has a few tools that deal with this
//! }
//! 
//! // create properties you want to share with pooled objects
//! pub struct MySwarmProperties;
//! 
//! fn main() {
//!     let swarm = Swarm::<MyPoolObject, MySwarmProperties>::new(10, MySwarmProperties);
//!     assert!(swarm.capacity() == 10);
//! }
//! ```

#[allow(dead_code)]
mod tests;
pub mod control;
pub mod types;
pub mod tools;

use control::SwarmControl;
use types::*;

/// The actual Swarm pool
pub struct Swarm<ItemType, Properties> {
    pool: Vec<ItemType>,
    spawns: Vec<Spawn>,
    free: Vec<Spawn>,
    len: usize,
    max: usize,
    order: Vec<usize>,

    pub properties: Properties,
}

impl<ItemType: Default + Copy, Properties> Swarm<ItemType, Properties> {

    /// Create a new Swarm object pool
    /// 
    /// The maximum `capacity` of the pool is defined here and cannot be changed aferwards, 
    /// This is the number of instances you can span.
    /// 
    /// You can add custom swarm `properties` for sharing values between spawned
    /// instances while iterating over these spawns.
    /// 
    /// # Example
    /// ```
    /// extern crate swarm;
    /// use swarm::Swarm;
    /// use swarm::tools::byte_str::ByteStr;
    /// 
    /// // create an object you want to pool
    /// // Swarm uses Copy and therfore only accepts Sized properties!
    /// // This means types such as String and Vec aren't allowed
    /// // The tools module has a few tools that deal with this
    /// #[derive(Default, Copy, Clone)]     
    /// pub struct MyPoolObject {           
    ///     pub name: ByteStr,              
    ///     pub value: usize,               
    /// }
    /// 
    /// // create properties you want to share with pooled objects
    /// pub struct MySwarmProperties;
    /// 
    /// fn main() {
    ///     let swarm = Swarm::<MyPoolObject, MySwarmProperties>::new(10, MySwarmProperties);
    ///     assert!(swarm.capacity() == 10);
    /// }
    /// ```
    pub fn new(capacity: usize, properties: Properties) -> Self {
        let mut spawns = Vec::<Spawn>::with_capacity(capacity);
        let mut order = Vec::<usize>::with_capacity(capacity);

        for i in 0..capacity { 
            let tag = Spawn::new(i);
            spawns.push(tag);
            order.push(i);
        }

        Swarm { 
            pool: vec![ItemType::default(); capacity],
            spawns,
            free: Vec::<Spawn>::with_capacity(capacity),
            len: 0,
            max: capacity,
            order,
            properties,
        }
    }
    
    pub(crate) fn control(&mut self) -> SwarmControl<ItemType, Properties> {
        SwarmControl {
            order: &mut self.order,
            pos: 0,
            len: self.len,
            max: &self.max, 
            spawns: &mut self.spawns, 
            free: &mut self.free,

            pool: &mut self.pool, 
            properties: &mut self.properties,
        }
    }

    /// Create a new pool instance. 
    /// Spawns are included in the update loop provided by Swarm
    /// 
    /// # Example
    /// ```
    /// extern crate swarm;
    /// use swarm::Swarm;
    /// use swarm::tools::byte_str::ByteStr;
    /// 
    /// // create an object you want to pool
    /// #[derive(Default, Copy, Clone)] 
    /// pub struct MyPoolObject {
    ///     pub name: ByteStr,
    ///     pub value: usize,
    /// }
    /// 
    /// // create properties you want to share with pooled objects
    /// pub struct MySwarmProperties;
    /// 
    /// fn main() {
    ///     let mut swarm = Swarm::<MyPoolObject, MySwarmProperties>::new(10, MySwarmProperties);
    ///     let spawn = swarm.spawn();
    /// 
    ///     assert!(spawn.is_some());
    ///     assert_eq!(swarm.count(), 1);
    /// }
    /// ```
    pub fn spawn(&mut self) -> Option<Spawn> {
        let mut ctx = self.control();
        let result = ctx.spawn();
        self.len = ctx.len;
        result
    }

    /// Remove a spawn from the swarm pool
    /// 
    /// # Example
    /// ```
    /// extern crate swarm;
    /// use swarm::Swarm;
    /// use swarm::tools::byte_str::ByteStr;
    /// 
    /// // create an object you want to pool
    /// #[derive(Default, Copy, Clone)] 
    /// pub struct MyPoolObject {
    ///     pub name: ByteStr,
    ///     pub value: usize,
    /// }
    /// 
    /// // create properties you want to share with pooled objects
    /// pub struct MySwarmProperties;
    /// 
    /// fn main() {
    ///     let mut swarm = Swarm::<MyPoolObject, MySwarmProperties>::new(10, MySwarmProperties);
    ///     let spawn = swarm.spawn();
    /// 
    ///     assert!(spawn.is_some());
    ///     assert_eq!(swarm.count(), 1);
    /// 
    ///     swarm.kill(&spawn.unwrap());
    ///     assert_eq!(swarm.count(), 0);
    /// }
    /// ```
    pub fn kill(&mut self, target: &Spawn) {
        let mut ctx = self.control();
        let reset_order = target.pos();
        ctx.kill(target);
        self.len = ctx.len;
        self.order[reset_order] = reset_order;
    }

    /// Returns a spawn reference object from an object position within the pool
    pub fn find_spawn(&self, pos: &ObjectPosition) -> Spawn {
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

    /// Returns a mutable reference to an object from the Swarm pool.
    /// The supplied object position points out which object to return.
    /// 
    /// NOTE: when killing objects the order of object positions change,
    /// Use fetch() instead of fetch_raw() if you are going to kill spawns 
    /// at any point in your code.
    pub fn fetch_raw(&mut self, pos: &ObjectPosition) -> &mut ItemType { 
        &mut self.pool[*pos]
    }

    /// Returns the number of spawned instances currently availeble
    pub fn count(&self) -> usize { self.len }

    /// Returns the maximum number of instances that can be spawned
    pub fn capacity(&self) -> usize { self.max }
    

    // update iterators

    /// Loops through all spawned instances and returns them via a callback
    /// handler. The callback handler is supplied with a mutable reference of these
    /// instances so that the object data of each looped instance can be changed.
    pub fn for_each(&mut self, handler: ForEachHandler<ItemType>) {
        let len = self.len;
        let mut i = 0;

        while &i < &len {
            handler(&mut self.pool[i]);
            i += 1;
        }
    }
    
    /// Loops through all spawned instances and returns their object position via a 
    /// callback handler. The callback handler also hands out a mutable reference to
    /// the object pool and the swarm properties object.
    /// 
    /// NOTE: This methode is quite fast (faster than the foreach or update methodes), but
    /// at the cost that is uses object positions. When an object is killed, the order 
    /// of some object positions are changed. Because of that, no tools for spawning nor 
    /// killing are supplied, which makes it impossible to do so within this loop.
    pub fn for_all(&mut self, handler: ForAllHandler<ItemType, Properties>) {
        let len = self.len;
        let mut i = 0;

        while &i < &len {
            handler(&i, &mut self.pool, &mut self.properties);
            i += 1;
        }
    }

    /// Loops through all spawned instances and returns a `SwarmControl` object via a 
    /// callback handler. The swarm control objects lets you edit the currently updated object
    /// as well as spawning and killing instances.
    /// 
    /// NOTE: This methode is slower then the other loops, but gives you full control over
    /// the swarm.
    pub fn update(&mut self, handler: UpdateHandler<ItemType, Properties>) {
        let mut i = 0;
        let len = self.len;
        let mut ctx = self.control();

        while &i < &len {
            ctx.pos = ctx.order[*&i];
            handler(&mut ctx);
            ctx.order[*&i] = i; 
            i += 1;
        }
        self.len = ctx.len;
    }
}