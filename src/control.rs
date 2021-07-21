//! Controling a swarm during the swarm.update() loop

use super::types::*;

pub struct SwarmControl<'a, T, P> {
    pub(crate) max: &'a usize,
    pub(crate) spawns: &'a mut Vec<Spawn>,
    pub(crate) free: &'a mut Vec<Spawn>,
    pub(crate) len: usize,
    pub(crate) order: &'a mut Vec<usize>,
    pub(crate) pos: usize,
    
    pub pool: &'a mut Vec<T>,
    pub properties: &'a mut P,
}

impl<'a, T: Default + Copy, P> SwarmControl<'a, T, P> {

    pub fn target(&mut self) -> &mut T {
        &mut self.pool[self.pos]
    }

    pub fn target_spawn(&self) -> Spawn {
        self.spawns[self.pos].mirror()
    }

    pub fn head(&self) -> ObjectPosition {
        self.order[self.pos]
    }

    pub fn fetch(&mut self, pos: &ObjectPosition) -> &mut T {
        &mut self.pool[*pos]
    }

    pub fn spawn_at(&self, pos: &ObjectPosition) -> Spawn {
        self.spawns[*pos].mirror()
    }

    pub fn kill_current(&mut self) {
        self.kill(&self.spawns[self.pos].mirror())
    }

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

    pub fn kill(&mut self, target: &Spawn) {
        target.0.borrow_mut().active = false;
    
        let last_pos = self.len - 1;
        let target_pos = target.pos();
    
        if self.len > 1 && target_pos < last_pos {
            
            let last_val = self.pool[last_pos];
            let target_val = self.pool[target_pos];
    
            // swap content to back
            self.pool[target_pos] = last_val;
            self.pool[last_pos] = target_val;
    
            // swap spawns equally
            self.spawns[target_pos] = self.spawns[last_pos].mirror();
            self.spawns[last_pos] = target.mirror();
    
            // set swapped spawn values to point to their own location
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