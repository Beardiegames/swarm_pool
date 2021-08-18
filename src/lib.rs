//! An object pooling system optimized for perfomance.
//! 
//! The pooling system manages object instances of a cutom type,
//! and provides update loops to iterate over them.
//! 
//! In order to create a new swarm pool, you need to define what your `pool object` and `swarm properties` types
//! are going to look like. Your `pool object` must at leas implement the Default and Clone traits 
//! from the standard library. The `swarm properties`, on the other hand, does not depend on any traits.
//! 
//! # Basic swarm setup example
//! ```
//! extern crate swarm_pool;
//! use swarm_pool::Swarm;
//! 
//! // create an object you want to pool
//! #[derive(Default, Clone)]     
//! pub struct MyPoolObject { 
//!     pub name: &'static str,
//!     pub value: usize,               
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
//! 
//! The swarm is now ready to be used. First of all we need to Spawn new pool instances. In reality all
//! objects in the pool are allready created and are waiting to be used. This means that all objects (
//! from 0 up to, but not including, the maximum capacity) can be accessed through the fetch() methode.
//! The difference between spawned and non-spawned pool objects is that spawned object are included in all
//! of the Swarm pools iterator methodes and non-spawned object are not.
//!
//! # Spawning and looping
//! ```
//! # extern crate swarm_pool;
//! # use swarm_pool::Swarm;

//! # #[derive(Default, Clone)]
//! # pub struct MyPoolObject {      
//! #     pub name: &'static str,     
//! #     pub value: usize,    
//! # }

//! let mut swarm = Swarm::<MyPoolObject, _>::new(10, ());
//! let spawn1 = swarm.spawn().unwrap();
//! let spawn2 = swarm.spawn().unwrap();
//!   
//! assert_eq!(swarm.fetch_ref(&spawn1).value, 0);
//! assert_eq!(swarm.fetch_ref(&spawn2).value, 0);
//!
//! swarm.for_each(|obj| {
//!     obj.value = 42;
//! });
//!
//! assert_eq!(swarm.fetch_ref(&spawn1).value, 42);
//! assert_eq!(swarm.fetch_ref(&spawn2).value, 42);
//! ```
//!
//! The real power of this library is not just looping through a few object instances, it is controlling and cross referencing them.
//! There are 2 powerful methodes that can be used to do so: `Swarm.for_all()` and `Swarm.update()`.
//! Both have their advantages and disadvantages, `for_all` loop is fast (equal to a standard vec for loop) but cannot spawn nor kill
//! pool objects, `update` is easy to use, gives full control, but is slow (less than half the speed).
//!
//! # Cross referencing using for_all & update
//! ```
//! # extern crate swarm_pool;
//! # use swarm_pool::{ Swarm, Spawn };

//! # #[derive(Default, Clone)]
//! # pub struct MyPoolObject {      
//! #     pub name: &'static str,     
//! #     pub value: usize,          
//! # }

//! // change properties to contain references to our spawned pool objects
//! pub struct MySwarmProperties { 
//!     john: Option<Spawn>, 
//!     cristy: Option<Spawn>,
//! }
//!
//! let properties = MySwarmProperties { john: None, cristy: None };
//! 
//! let mut swarm = Swarm::<MyPoolObject, MySwarmProperties>::new(10, properties);
//! let s_john = swarm.spawn().unwrap();
//! let s_cristy = swarm.spawn().unwrap();
//!
//! swarm.properties.john = Some(s_john.mirror());
//! swarm.properties.cristy = Some(s_cristy.mirror());
//!
//! swarm.fetch(&s_john).name = "John";
//! swarm.fetch(&s_cristy).name = "Cristy";
//!
//! swarm.for_all(|target, pool, properties| {
//!
//!     // john tells critsy to have a value of 2
//!     if pool[*target].name == "John" { 
//!         if let Some(cristy) = &properties.cristy {
//!             pool[cristy.pos()].value = 2; 
//!         }
//!     }
//!     // cristy tells john to have a value of 1
//!     if pool[*target].name == "Cristy" { 
//!         if let Some(john) = &properties.john {
//!             pool[john.pos()].value = 1; 
//!         }
//!     }
//! });
//!
//! assert_eq!(swarm.fetch_ref(&s_john).value, 1);
//! assert_eq!(swarm.fetch_ref(&s_cristy).value, 2);
//!
//! swarm.update(|ctl| {
//!     let name = ctl.target().name;
//!     let cristy = ctl.properties.cristy.as_ref().unwrap().mirror();
//!     let john = ctl.properties.john.as_ref().unwrap().mirror();
//!
//!     // john tells critsy to have a value of 4
//!     if name == "John" { 
//!         ctl.fetch(&cristy).value = 4; 
//!     }
//!     // cristy tells john to have a value of 5
//!     if name == "Cristy" { 
//!         ctl.fetch(&john).value = 5; 
//!     }
//! });
//!
//! assert_eq!(swarm.fetch_ref(&s_john).value, 5);
//! assert_eq!(swarm.fetch_ref(&s_cristy).value, 4);
//! ```
//!
//! There are many more functionalities included in the Swarm and SwarmControl types. 
//! The documentation on the examples above or other functionalities this library provides are more in depth
//! and should be read, for writing them out was a lot of work ;)


#[allow(dead_code)]
mod tests;
pub mod control;
pub mod types;
//pub mod tools;

use control::SwarmControl;
pub use types::*;

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

impl<ItemType: Default + Clone, Properties> Swarm<ItemType, Properties> {

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
    /// extern crate swarm_pool;
    /// use swarm_pool::Swarm;
    /// 
    /// // create an object you want to pool
    /// // Swarm uses Copy and therfore only accepts Sized properties!
    /// // This means types such as String and Vec aren't allowed
    /// // The tools module has a few tools that deal with this
    
    /// #[derive(Default, Clone)]     
    /// pub struct MyPoolObject {           
    ///     pub name: &'static str,              
    ///     pub value: usize,               
    /// }
    /// 
    /// // create properties you want to share with pooled objects
    /// pub struct MySwarmProperties;
    /// 
    /// let swarm = Swarm::<MyPoolObject, MySwarmProperties>::new(10, MySwarmProperties);
    /// assert!(swarm.capacity() == 10);
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
    /// Create a new spawn for every item in the `items` list and gives
    /// it that value.
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
    ///     assert_eq!(*swarm.fetch_raw(&0), 5);
    /// ```
    pub fn populate(&mut self, items: &[ItemType]) {
        for item in items {
            if let Some(s) = self.spawn() {
                *self.fetch(&s) = item.clone();
            }
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
    /// Spawns are pool instances that will be included in the update loops 
    /// provided by Swarm, as long as they are active and not killed yet.
    /// 
    /// Returns None if the pool reached it's maximum capacity and 
    /// therefore could not spawn new instances
    /// 
    /// # Example
    /// ```
    /// extern crate swarm_pool;
    /// use swarm_pool::Swarm;
    /// 
    /// // create an object you want to pool
    /// #[derive(Default, Clone)] 
    /// pub struct MyPoolObject {
    ///     pub name: &'static str,
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
        let mut ctl = self.control();
        let result = ctl.spawn();
        self.len = ctl.len;
        result
    }

    /// Remove a spawn instance from the swarm pool update loops
    /// 
    /// # Example
    /// ```
    /// extern crate swarm_pool;
    /// use swarm_pool::Swarm;
    /// 
    /// // create an object you want to pool
    /// #[derive(Default, Clone)] 
    /// pub struct MyPoolObject {
    ///     pub name: &'static str,
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
        let mut ctl = self.control();
        let reset_order = target.pos();
        ctl.kill(target);
        self.len = ctl.len;
        self.order[reset_order] = reset_order;
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
    pub fn enumerate(&mut self, handler: EnumerateHandler<ItemType>) {
        let len = self.len;
        let mut i = 0;

        while &i < &len {
            handler(&i, &mut self.pool[i]);
            i += 1;
        }
    }

    /// Loops through all spawned instances and returns them via a callback
    /// handler. The callback handler is supplied with a mutable reference of these
    /// instances so that the object data of each looped instance can be changed.
    ///
    /// # Example
    /// ```
    /// extern crate swarm_pool;
    /// use swarm_pool::{ Swarm, Spawn };
    ///
    /// // create an object you want to pool
    /// #[derive(Default, Clone)] 
    /// pub struct MyPoolObject {
    ///     pub value: usize,
    /// }
    /// let mut swarm = Swarm::<MyPoolObject, _>::new(10, ());
    /// let spawn1 = swarm.spawn().unwrap();
    /// let spawn2 = swarm.spawn().unwrap();
    ///   
    /// assert_eq!(swarm.fetch_ref(&spawn1).value, 0);
    /// assert_eq!(swarm.fetch_ref(&spawn2).value, 0);
    ///
    /// swarm.for_each(|obj| {
    ///     obj.value = 42;
    /// });
    ///
    /// assert_eq!(swarm.fetch_ref(&spawn1).value, 42);
    /// assert_eq!(swarm.fetch_ref(&spawn2).value, 42);
    /// ```
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
    ///
    /// # Examples
    /// ## Basic usecase
    /// ```
    /// extern crate swarm_pool;
    /// use swarm_pool::{ Swarm, Spawn };
    ///
    /// // create an object you want to pool
    /// #[derive(Default, Clone)] 
    /// pub struct MyPoolObject {
    ///     pub name: &'static str,
    ///     pub value: usize,
    /// }
    /// 
    /// let mut swarm = Swarm::<MyPoolObject, _>::new(10, ());
    /// let spawn1 = swarm.spawn().unwrap();
    /// let spawn2 = swarm.spawn().unwrap();
    ///
    /// assert_eq!(swarm.fetch_ref(&spawn1).value, 0);
    /// assert_eq!(swarm.fetch_ref(&spawn2).value, 0);
    ///
    /// swarm.for_all(|target, list, _props| {
    ///     list[*target].value = *target + 1;
    /// });
    ///
    /// assert_eq!(swarm.fetch_ref(&spawn1).value, 1);
    /// assert_eq!(swarm.fetch_ref(&spawn2).value, 2);
    /// ```
    ///
    /// ## Using shared properties
    /// ```
    /// # extern crate swarm_pool;
    /// # use swarm_pool::{ Swarm, Spawn };
    /// # // create an object you want to pool
    /// # #[derive(Default, Clone)] 
    /// # pub struct MyPoolObject { pub name: &'static str, pub value: usize, }
    /// # //
    /// // create properties you want to share with pooled objects
    /// pub struct MySwarmProperties { 
    ///     counter: usize 
    /// }
    /// 
    /// let properties = MySwarmProperties { counter: 0 };
    ///
    /// let mut swarm = Swarm::<MyPoolObject, MySwarmProperties>::new(10, properties);
    /// let spawn1 = swarm.spawn().unwrap();
    /// let spawn2 = swarm.spawn().unwrap();
    ///
    /// swarm.for_all(|target, list, props| {
    ///     props.counter += 1;
    ///     list[*target].value = props.counter;
    /// });
    ///
    /// assert_eq!(swarm.properties.counter, 2);
    /// assert_eq!(swarm.fetch_ref(&spawn1).value, 1);
    /// assert_eq!(swarm.fetch_ref(&spawn2).value, 2);
    /// ```
    ///
    /// ## Cross referencing between objects
    /// ```
    /// # extern crate swarm_pool;
    /// # use swarm_pool::{ Swarm, Spawn };
    /// # #[derive(Default, Clone)] 
    /// # pub struct MyPoolObject { pub name: &'static str, pub value: usize }
    /// # //
    /// // create properties you want to share with pooled objects
    /// pub struct MySwarmProperties { 
    ///     john: Option<Spawn>, 
    ///     cristy: Option<Spawn>,
    /// }
    ///
    /// let properties = MySwarmProperties { john: None, cristy: None };
    /// 
    /// let mut swarm = Swarm::<MyPoolObject, MySwarmProperties>::new(10, properties);
    /// let s_john = swarm.spawn().unwrap();
    /// let s_cristy = swarm.spawn().unwrap();
    ///
    /// swarm.properties.john = Some(s_john.mirror());
    /// swarm.properties.cristy = Some(s_cristy.mirror());
    ///
    /// swarm.fetch(&s_john).name = "John";
    /// swarm.fetch(&s_cristy).name = "Cristy";
    ///
    /// swarm.for_all(|target, list, props| {
    ///
    ///     // john tells critsy to have a value of 2
    ///     if list[*target].name == "John" { 
    ///         if let Some(cristy) = &props.cristy {
    ///             list[cristy.pos()].value = 2; 
    ///         }
    ///     }
    ///     // cristy tells john to have a value of 1
    ///     if list[*target].name == "Cristy" { 
    ///         if let Some(john) = &props.john {
    ///             list[john.pos()].value = 1; 
    ///         }
    ///     }
    /// });
    ///
    /// assert_eq!(swarm.fetch_ref(&s_john).value, 1);
    /// assert_eq!(swarm.fetch_ref(&s_cristy).value, 2);
    /// ```
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
    /// 
    ///
    /// # Examples
    /// ##  Cross Referencing between objects
    /// ```
    /// # extern crate swarm_pool;
    /// # use swarm_pool::{ Swarm, Spawn };
    /// // create an object you want to pool
    /// #[derive(Default, Clone)] 
    /// pub struct MyPoolObject { 
    ///     pub name: &'static str,
    ///     pub value: usize,
    /// }
    /// 
    /// // create properties you want to share with pooled objects
    /// pub struct MySwarmProperties {
    ///     john: Option<Spawn>,
    ///     cristy: Option<Spawn>,
    /// }
    ///
    /// let properties = MySwarmProperties {
    ///     john: None,
    ///     cristy: None,
    /// };
    /// 
    /// let mut swarm = Swarm::<MyPoolObject, MySwarmProperties>::new(10, properties);
    ///
    /// let s_john = swarm.spawn().unwrap();
    /// let s_cristy = swarm.spawn().unwrap();
    ///
    /// swarm.properties.john = Some(s_john.mirror());
    /// swarm.properties.cristy = Some(s_cristy.mirror());
    ///
    /// swarm.fetch(&s_john).name = "John";
    /// swarm.fetch(&s_cristy).name = "Cristy";
    ///
    /// swarm.update(|ctl| {
    ///     let name = ctl.target().name;
    ///     let cristy = ctl.properties.cristy.as_ref().unwrap().mirror();
    ///     let john = ctl.properties.john.as_ref().unwrap().mirror();
    ///
    ///     // john tells critsy to have a value of 2
    ///     if name == "John" { 
    ///         ctl.fetch(&cristy).value = 2; 
    ///     }
    ///     // cristy tells john to have a value of 1
    ///     if name == "Cristy" { 
    ///         ctl.fetch(&john).value = 1; 
    ///     }
    /// });
    ///
    /// assert_eq!(swarm.fetch_ref(&s_john).value, 1);
    /// assert_eq!(swarm.fetch_ref(&s_cristy).value, 2);
    /// ```
    pub fn update(&mut self, handler: UpdateHandler<ItemType, Properties>) {
        let mut i = 0;
        let len = self.len;
        let mut ctl = self.control();

        while &i < &len {
            ctl.pos = ctl.order[*&i];
            handler(&mut ctl);
            ctl.order[*&i] = i; 
            i += 1;
        }
        self.len = ctl.len;
    }
}
