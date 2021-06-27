

use super::*;


pub enum ComponentError { 
    ZeroNotAllowed,
}

pub struct Component(std::num::NonZeroU8);

impl Component {
    pub fn new(value: u8) -> Self {
        if value > 254 {
            panic!("parameter from(value: u8) cannot be bigger than 254!");
        }
        match std::num::NonZeroU8::new(value + 1) {
            Some(v) => Component (v),
            None => panic!("parameter from(value: u8) does not allow a value of zero!"),
        }
    }
}

//pub type Component = core::num::NonZeroU8;

// pub trait Component: From<u8> {
//     fn convert(self) -> core::num::NonZeroU8 {
//         std::num::NonZeroU8::from(u8::from(self))
//     }
//     // fn convert(&self) -> Result<core::num::NonZeroU8, ComponentError> {
//     //     core::num::NonZeroU8::try_from(u8::from(self))
//     //         .map_err(|_e| ComponentError::ZeroValueNotAllowed)
//     // } 
// }

#[derive(Copy, Clone)]
pub struct Entity(pub(crate) u64);

impl Entity {

    pub fn clear(&mut self) { self.0 = 0; }

    pub fn add_component(&mut self, component: Component) {
        self.0 |= 1 << component.0.get();
    }

    pub fn remove_component(&mut self, component: Component) {
        self.0 &= !(1 << component.0.get());
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
