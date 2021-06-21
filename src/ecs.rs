

use super::*;

pub struct RequiredComponents(pub(crate) u64);

impl RequiredComponents {
    pub const fn new(components: &[u8]) -> Self {
        let mut req = RequiredComponents(0);
        let i =0;
        while i < components.len() {
            let c = components[i] + 1;
            let bit = 1 << c;
            req.0 = req.0 | bit;
        }   
        req
    }
}

#[derive(Copy, Clone)]
pub struct Entity(pub(crate) u64);

impl Entity {

    pub fn clear(&mut self) { self.0 = 0; }

    pub fn add_component(&mut self, component: u8) {
        self.0 |= 1 << (component + 1);
    }

    pub fn remove_component(&mut self, component: u8) {
        self.0 &= !(1 << (component + 1));
    }

}

// #[derive(Clone)]
// pub struct Component(usize);

// impl Component {
//     pub const fn new(num: usize) -> Self { //} Result<Self, &'static str> {
//         Component (num)
//         // match num > 0 || num < 64 {
//         //     true => Ok(Component (num)),
//         //     false => Err("Component Err! parameter 'num' out of bounds: 
//         //     Components should have a numeric value greater than zero and 
//         //     smaller than 32!"),
//         // }
//     }
// }


// pub trait Component: Into<u8> + Copy {
//     fn value(self) -> u8 {
//         self.into() + 1
//     }
// }

pub trait System<T: Default + Copy> {

    const COMPONENTS: RequiredComponents;

    fn update(&mut self, spawn: &Spawn, swarm: &mut Swarm<T>);

    fn run(&mut self, swarm: &mut Swarm<T>) {
        for i in 0..swarm.len {
            if Self::COMPONENTS.0 == swarm.entities[i].0 & Self::COMPONENTS.0 {
                self.update(&i, swarm);
            }
        }
    }
}
