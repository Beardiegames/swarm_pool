#[allow(dead_code)]
mod tests;
mod context;
mod types;
mod tools;

use context::SwarmContext;
use types::*;

pub struct Swarm<T, P> {
    pool: Vec<T>,
    spawns: Vec<Spawn>,
    free: Vec<Spawn>,
    len: usize,
    max: usize,
    order: Vec<usize>,

    pub properties: P,
}

/// Swarm is an object pooling system optimized for perfomance.
/// 
/// The pooling system manages objects of your own cutom type,
/// and iterate over them without too much performance.
/// 
/// ```
/// extern crate swarm;
/// use swarm::Swarm;
/// 
/// #[derive(Default, Copy, Clone)]     // Your pool object must implement these
/// pub struct Minion {                 
///     name: [u8; 6],                  // Swarm uses Copy and therfore only accepts Sized data!
///     value: usize,                   // This means types such as String and Vec aren't allowed
/// }                                   // In the tools module are a few tools that deal with this
///
/// impl Minion {
///     pub fn add_one(&mut self) {
///         self.value += 1;
///     }
/// }
/// 
/// pub struct SwarmProperties;         // Data you want to share with pool instances
/// 
/// fn main() {
///     let swarm = Swarm::<Minion, SwarmProperties>::new(10, SwarmProperties);
///     assert!(swarm.max_size() == 10);
/// }
/// ```
impl<T: Default + Copy, P> Swarm<T, P> {

    pub fn new(size: usize, properties: P) -> Self {
        let mut spawns = Vec::<Spawn>::with_capacity(size);
        let mut order = Vec::<usize>::with_capacity(size);

        for i in 0..size { 
            let tag = Spawn::new(i);
            spawns.push(tag);
            order.push(i);
        }

        Swarm { 
            pool: vec![T::default(); size],
            spawns,
            free: Vec::<Spawn>::with_capacity(size),
            len: 0,
            max: size,
            order,
            properties,
        }
    }
    
    pub fn context(&mut self) -> SwarmContext<T, P> {
        SwarmContext {
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

    pub fn spawn(&mut self) -> Option<Spawn> {
        let mut ctx = self.context();
        let result = ctx.spawn();
        self.len = ctx.len;
        result
    }

    pub fn kill(&mut self, target: &Spawn) {
        let mut ctx = self.context();
        let reset_order = target.pos();
        ctx.kill(target);
        self.len = ctx.len;
        self.order[reset_order] = reset_order;
    }

    pub fn spawn_at(&self, pos: &ObjectPosition) -> Spawn {
        self.spawns[*pos].mirror()
    }

    pub fn get(&mut self, spawn: &Spawn) -> &mut T { 
        &mut self.pool[spawn.0.borrow().pos]
    }

    pub fn get_ref(&self, spawn: &Spawn) -> &T { 
        &self.pool[spawn.0.borrow().pos]
    }

    pub fn get_raw(&mut self, pos: &ObjectPosition) -> &mut T { 
        &mut self.pool[*pos]
    }

    pub fn count(&self) -> usize { self.len }

    pub fn max_size(&self) -> usize { self.max }
    

    // update iterators

    pub fn for_each(&mut self, handler: ForEachHandler<T>) {
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

    pub fn update(&mut self, handler: UpdateHandler<T, P>) {
        let mut i = 0;
        let len = self.len;
        let mut ctx = self.context();

        while &i < &len {
            ctx.pos = ctx.order[*&i];
            handler(&mut ctx);
            ctx.order[*&i] = i; 
            i += 1;
        }
        self.len = ctx.len;
    }
}