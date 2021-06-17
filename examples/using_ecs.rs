#[cfg(test)]

use swarm::ecs::requirements;

use swarm::{ Swarm, SwarmError, Spawn };
use swarm::ecs::{ Entity, Component, System };

fn main() {

}

#[derive(Default, Copy, Clone)]
pub struct Minion {
    health: i8,
    strength: i8,
    all: i8,
}

pub enum Components { Health, Strength, }

impl Into<Component> for Components {
    fn into(self) -> Component {
        self as Component + 1
    }
}


pub struct HealthSystem ;

impl System<Minion> for HealthSystem {
    const COMPONENTS: Entity = Entity::from_requirements(swarm::requirements!(Components::Health.into()));

    fn update(&mut self, spawn: &Spawn, swarm: &mut Swarm<Minion>) {
        swarm.get_mut(spawn).health += 1;
    }
}


// pub struct StrengthSystem;

// impl System<Minion> for StrengthSystem {
//     fn update(&mut self, spawn: &Spawn, swarm: &mut Swarm<Minion>) {
//         swarm.get_mut(spawn).strength += 1;
//     }
// }


// pub struct AllSystem;

// impl System<Minion> for AllSystem {
//     fn update(&mut self, spawn: &Spawn, swarm: &mut Swarm<Minion>) {
//         swarm.get_mut(spawn).all += 1;
//     }
// }

#[test]
fn ecs_setup() {
    let mut swarm = Swarm::<Minion>::new(10);

    let required_components: &[Component] = &[ Components::Health.into() ];

    let mut system = HealthSystem;

    let spawn = swarm.spawn().unwrap();
    assert_eq!(swarm.get_ref(&spawn).health, 0);

    system.run(&mut swarm);
    // spawn is not updated because it does not have the health components
    assert_eq!(swarm.get_ref(&spawn).health, 0);

    swarm.add_component(&spawn, Components::Health.into());
    system.run(&mut swarm);
    assert_eq!(swarm.get_ref(&spawn).health, 1);
}


// #[test]
// fn using_multiple_systems() {
//     let mut swarm = Swarm::<Minion>::new(10);

//     let health_required_components: &[Component] = &[ Components::Health.into() ];
//     let strength_required_components: &[Component] = &[ Components::Strength.into() ];
//     let all_required_components: &[Component] = &[ Components::Health.into(), Components::Strength.into() ];
    
//     let mut health_system = HealthSystem;
//     let mut strength_system = StrengthSystem;
//     let mut all_system = AllSystem;

//     let spawn1 = swarm.spawn().unwrap();
//     swarm.add_component(&spawn1, Components::Health.into());

//     let spawn2 = swarm.spawn().unwrap();
//     swarm.add_component(&spawn2, Components::Strength.into());

//     let spawn3 = swarm.spawn().unwrap();
//     swarm.add_component(&spawn3, Components::Health.into());
//     swarm.add_component(&spawn3, Components::Strength.into());

//     health_system.run(&mut swarm);
//     assert_eq!(swarm.get_ref(&spawn1).health, 1);
//     assert_eq!(swarm.get_ref(&spawn2).health, 0);
//     assert_eq!(swarm.get_ref(&spawn3).health, 1);

//     strength_system.run(&mut swarm);
//     assert_eq!(swarm.get_ref(&spawn1).strength, 0);
//     assert_eq!(swarm.get_ref(&spawn2).strength, 1);
//     assert_eq!(swarm.get_ref(&spawn3).strength, 1);

//     all_system.run(&mut swarm);
//     assert_eq!(swarm.get_ref(&spawn1).all, 0);
//     assert_eq!(swarm.get_ref(&spawn2).all, 0);
//     assert_eq!(swarm.get_ref(&spawn3).all, 1);
// }

// #[test]
// fn a_system_can_have_properties() {
//     assert!(false);
// }

// #[test]
// fn entities_can_communitcate() {
//     assert!(false);
// }


