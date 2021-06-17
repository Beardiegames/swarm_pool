

use super::*;

#[derive(Copy, Clone)]
pub struct Entity(pub(crate) u32);

impl Entity {

    pub const fn from_requirements(req: u32) -> Entity {
        Entity(req)
    }

    pub fn clear(&mut self) { self.0 = 0; }

    pub fn add_component(&mut self, component: Component) {
        self.0 |= 1 << component;
    }

    pub fn remove_component(&mut self, component: Component) {
        self.0 &= !(1 << component);
    }

}

pub type Component = usize;

pub type RequiredComponents = Entity;

const fn setup_requirentments(component: Component) -> Entity {
    let mut bits: u32 = 0;
    bits |= 1 << component;
    Entity (bits)
}


//pub type UpdateSystem<T> = fn(&Spawn, &mut Swarm<T>);

pub trait System<T: Default + Copy> {

    const COMPONENTS: Entity;

    //fn requirements(&mut self) -> &mut RequiredComponents;

    fn update(&mut self, spawn: &Spawn, swarm: &mut Swarm<T>);

    // fn new(require_components: &[Component], updater: UpdateSystem<T>) -> Self {
    //     let mut require = Entity (0);
    //     for c in require_components {
    //         require.add_component(*c);
    //     }
    //     System { require, updater }
    // }

    // fn add_requirement(&mut self, component: Component) {
    //     self.requirements().add_component(component);
    // }

    fn run(&mut self, swarm: &mut Swarm<T>) {
        for i in 0..swarm.len {
            if Self::COMPONENTS.0 == swarm.entities[i].0 & Self::COMPONENTS.0 {
                self.update(&i, swarm);
            }
        }
    }
}
