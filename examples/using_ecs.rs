#[cfg(test)]

use swarm::{ Swarm, SwarmError, Spawn };
use swarm::ecs::{ Entity, Component, SystemBuilder, System };
use std::convert::TryFrom;

fn main() {

}

#[derive(Default, Copy, Clone)]
pub struct Minion {
    health: i8,
    strength: i8,
    both: i8,
}


#[derive(Copy, Clone)]
#[repr(u8)]
pub enum GameComponents { Health, Strength, Move }


pub struct HealthSystem;

impl System<Minion> for HealthSystem {
    fn update(&mut self, spawn: &swarm::Spawn, swarm: &mut swarm::Swarm<Minion>) {
        swarm.get_mut(spawn).health += 1;
    }
}

pub struct StrengthSystem;

impl System<Minion> for StrengthSystem {
    fn update(&mut self, spawn: &swarm::Spawn, swarm: &mut swarm::Swarm<Minion>) {
        swarm.get_mut(spawn).strength += 1;
    }
}


pub struct BothSystem;

impl System<Minion> for BothSystem {
    fn update(&mut self, spawn: &swarm::Spawn, swarm: &mut swarm::Swarm<Minion>) {
        swarm.get_mut(spawn).both += 1;
    }
}

#[test]
fn ecs_setup() {
    let mut swarm = Swarm::<Minion>::new(10);
    let mut system = SystemBuilder::new(HealthSystem) 
        .requires_component(Component::new(GameComponents::Health as u8))
        .build();

    let spawn = swarm.spawn().unwrap();
    assert_eq!(swarm.get_ref(&spawn).health, 0);

    system.run(&mut swarm);
    // spawn is not updated because it does not have the health component
    assert_eq!(swarm.get_ref(&spawn).health, 0);

    swarm.add_component(&spawn, Component::new(GameComponents::Health as u8));
    system.run(&mut swarm);
    assert_eq!(swarm.get_ref(&spawn).health, 1);
}


#[test]
fn using_multiple_systems() {
    let mut swarm = Swarm::<Minion>::new(10);

    let mut health_system =  SystemBuilder::new(HealthSystem)
        .requires_component(Component::new(GameComponents::Health as u8))
        .build();
    let mut strength_system =  SystemBuilder::new(StrengthSystem)
        .requires_component(Component::new(GameComponents::Strength as u8))
        .build();
    let mut both_system =  SystemBuilder::new(BothSystem)
        .requires_component(Component::new(GameComponents::Health as u8))
        .requires_component(Component::new(GameComponents::Strength as u8))
        .build();

    let spawn1 = swarm.spawn().unwrap();
    swarm.add_component(&spawn1, Component::new(GameComponents::Health as u8));

    let spawn2 = swarm.spawn().unwrap();
    swarm.add_component(&spawn2, Component::new(GameComponents::Strength as u8));

    let spawn3 = swarm.spawn().unwrap();
    swarm.add_component(&spawn3, Component::new(GameComponents::Health as u8));
    swarm.add_component(&spawn3, Component::new(GameComponents::Strength as u8));

    health_system.run(&mut swarm);
    assert_eq!(swarm.get_ref(&spawn1).health, 1);
    assert_eq!(swarm.get_ref(&spawn2).health, 0);
    assert_eq!(swarm.get_ref(&spawn3).health, 1);

    strength_system.run(&mut swarm);
    assert_eq!(swarm.get_ref(&spawn1).strength, 0);
    assert_eq!(swarm.get_ref(&spawn2).strength, 1);
    assert_eq!(swarm.get_ref(&spawn3).strength, 1);

    both_system.run(&mut swarm);
    assert_eq!(swarm.get_ref(&spawn1).both, 0);
    assert_eq!(swarm.get_ref(&spawn2).both, 0);
    assert_eq!(swarm.get_ref(&spawn3).both, 1);
}

// #[test]
// fn a_system_can_have_properties() {
//     assert!(false);
// }

// #[test]
// fn entities_can_communitcate() {
//     assert!(false);
// }


