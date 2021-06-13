use super::*;

#[derive(Copy, Clone)]
pub struct Entity(pub(crate) u32);

impl Entity {
    pub fn clear(&mut self) { self.0 = 0; }

    pub fn add_component(&mut self, component: Component) {
        self.0 |= 1 << component;
    }

    pub fn remove_component(&mut self, component: Component) {
        self.0 &= !(1 << component);
    }
}

pub type Component = usize;

pub struct System<T: Default + Copy> {
    require: Entity,
    updater: fn(&mut T),
}

impl<T:Default + Copy> System<T> {

    pub fn new(require_components: &[Component], updater: fn(&mut T)) -> Self {
        let mut require = Entity (0);
        for c in require_components {
            require.add_component(*c);
        }
        System { require, updater }
    }

    pub fn run(&mut self, swarm: &mut Swarm<T>) {
        for i in 0..swarm.len {
            if self.require.0 == swarm.entities[i].0 & self.require.0 {
                (self.updater)(&mut swarm.content[i]);
            }
        }
    }
}
