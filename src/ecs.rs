

use super::*;
use std::num::NonZeroU8;

pub type Component = NonZeroU8;

#[derive(Copy, Clone)]
pub struct Entity(pub(crate) u64);

impl Entity {

    pub fn clear(&mut self) { self.0 = 0; }

    pub fn add_component(&mut self, component: NonZeroU8) {
        self.0 |= 1 << component.get();
    }

    pub fn remove_component(&mut self, component: NonZeroU8) {
        self.0 &= !(1 << component.get());
    }

}

pub trait System<T: Default + Copy> {
    fn update(&mut self, spawn: &Spawn, swarm: &mut Swarm<T>);
}

pub struct EcsShell<T: Default + Copy> {
    components: Entity,
    system: Box<dyn System<T>>,
}
 
impl<T: Default + Copy> EcsShell<T> {

    pub fn run(&mut self, swarm: &mut Swarm<T>) {
        for i in 0..swarm.len {
            if self.components.0 == swarm.entities[i].0 & self.components.0 {
                self.system.update(&i, swarm);
            }
        }
    }
}

pub struct SystemBuilder<T: Default + Copy> {
    shell: EcsShell<T>,
}
 
impl<T: Default + Copy> SystemBuilder<T> {

    pub fn new<S: System<T> + 'static>(system: S) -> Self {
        SystemBuilder {
            shell: EcsShell { 
                components: Entity(0), 
                system: Box::new(system)
            }
        }
    }

    pub fn requires_component(mut self, component: Component) -> Self {
        self.shell.components.add_component(component);
        self
    }

    pub fn build(self) -> EcsShell<T> {
        self.shell
    }
}
