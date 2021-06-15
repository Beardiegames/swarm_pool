use super::*;

#[derive(Copy, Clone)]
pub struct Entity(pub(crate) u32);

impl Entity {

    const fn new() -> Self { Entity (0) }

    pub fn clear(&mut self) { self.0 = 0; }

    pub fn add_component(&mut self, component: Component) {
        self.0 |= 1 << component;
    }

    pub fn remove_component(&mut self, component: Component) {
        self.0 &= !(1 << component);
    }

}

impl From<[Component; 1]> for Entity {
    fn from(components: [Component; 1]) -> Entity {
        let mut require = Entity (0);
        for c in &components {
            require.0 |= 1 << c;
        }
        require
    }
}

pub type Component = usize;

pub type RequiredComponents = Entity;


//pub type UpdateSystem<T> = fn(&Spawn, &mut Swarm<T>);

pub trait System<T: Default + Copy> {

    fn requirements(&mut self) -> &mut RequiredComponents;

    fn update(&mut self, spawn: &Spawn, swarm: &mut Swarm<T>);

    // fn new(require_components: &[Component], updater: UpdateSystem<T>) -> Self {
    //     let mut require = Entity (0);
    //     for c in require_components {
    //         require.add_component(*c);
    //     }
    //     System { require, updater }
    // }

    fn add_requirement(&mut self, component: Component) {
        self.requirements().add_component(component);
    }

    fn run(&mut self, swarm: &mut Swarm<T>) {
        for i in 0..swarm.len {
            if self.requirements().0 == swarm.entities[i].0 & self.requirements().0 {
                self.update(&i, swarm);
            }
        }
    }
}
