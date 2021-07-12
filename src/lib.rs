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
pub type Update2Handler<T, P> = fn(&mut SwarmContext<T, P>);

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

pub struct SwarmContext<'a, T, P> {
    order: &'a mut Vec<usize>,
    pos: usize,
    len: usize,
    max: &'a usize,
    spawns: &'a mut Vec<Spawn>,
    free: &'a mut Vec<Spawn>,
    
    pub pool: &'a mut Vec<T>,
    pub properties: &'a mut P,
}

impl<'a, T: Default + Copy, P> SwarmContext<'a, T, P> {

    pub fn target(&mut self) -> &mut T {
        &mut self.pool[self.pos]
    }

    pub fn target_spawn(&self) -> Spawn {
        self.spawns[self.pos].mirror()
    }

    pub fn head(&self) -> ObjectPosition {
        self.order[self.pos]
    }

    pub fn get(&mut self, pos: &ObjectPosition) -> &mut T {
        &mut self.pool[*pos]
    }

    pub fn spawn(&mut self) -> Option<Spawn> {
        crate::spawn(self)
    }

    pub fn kill(&mut self, spawn: &Spawn) {
        crate::kill(spawn, self)
    }

    pub fn kill_current(&mut self) {
        crate::kill(&self.spawns[self.pos].mirror(), self)
    }

    pub fn spawn_at(&self, pos: &ObjectPosition) -> Spawn {
        self.spawns[*pos].mirror()
    }
}

pub fn spawn<T: Default + Clone, P> (ctx: &mut SwarmContext<T, P>) -> Option<Spawn> {
    if ctx.len < *ctx.max {

        ctx.len += 1;
        let pos = ctx.len - 1;
 
        if ctx.free.len() > 0 {
            ctx.free.pop().map(|s| { 
                s.0.borrow_mut().pos = pos; 
                s.0.borrow_mut().active = true;
                s 
            })
        } else {
            
            let s = &ctx.spawns[pos];
                s.0.borrow_mut().pos = pos;
                s.0.borrow_mut().active = true;

            Some(s.mirror())
        }
    } else {
        None
    }
}

pub fn kill<T: Default + Copy, P> (target: &Spawn, ctx: &mut SwarmContext<T, P>) {
    target.0.borrow_mut().active = false;
    
    let last_pos = ctx.len - 1;
    let target_pos = target.pos();

    if ctx.len > 1 && target_pos < last_pos {
        
        let last_val = ctx.pool[last_pos];
        let target_val = ctx.pool[target_pos];

        // swap content to back
        ctx.pool[target_pos] = last_val;
        ctx.pool[last_pos] = target_val;

        // swap spawns equally
        ctx.spawns[target_pos] = ctx.spawns[last_pos].mirror();
        ctx.spawns[last_pos] = target.mirror();

        // set swapped spawn values to point to their own location
        ctx.spawns[target_pos].0.borrow_mut().pos = target_pos;
        ctx.spawns[last_pos].0.borrow_mut().pos = last_pos;

        if target_pos > ctx.pos { ctx.order[target_pos] = last_pos; }
        if last_pos > ctx.pos { ctx.order[last_pos] = target_pos; }
    }

    // store and decrement size             
    if ctx.len > 0 { 
        ctx.free.push(target.mirror());
        ctx.len -= 1; 
    }
}

pub struct Swarm<T, P> {
    pool: Vec<T>,
    spawns: Vec<Spawn>,
    free: Vec<Spawn>,
    len: usize,
    max: usize,
    order: Vec<usize>,
    iter: usize,

    pub properties: P,
}

impl<T: Default + Copy, P> Swarm<T, P> {

    pub fn new(size: usize, properties: P) -> Self {
        let mut spawns = Vec::<Spawn>::with_capacity(size);
        let mut order = Vec::<usize>::with_capacity(size);

        for i in 0..size { 
            let tag = Spawn (
                Rc::new(RefCell::new(Tag{ id:i, pos:i, active:false }))
            );
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
            iter:0,

            properties,
        }
    }

    // pooling
    
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
        let result = crate::spawn(&mut ctx);
        self.len = ctx.len;
        result
    }

    pub fn kill(&mut self, target: &Spawn) {
        let mut ctx = self.context();
        let reset_order = target.pos();
        crate::kill(target, &mut ctx);
        self.len = ctx.len;
        self.order[reset_order] = reset_order;
    }

    // states & references

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

    pub fn update(&mut self, handler: Update2Handler<T, P>) {
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