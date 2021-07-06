#[cfg(test)]

use crate::*;
use crate as swarm;

#[derive(Default, Clone, Debug)]
pub struct Minion {
    name: String,
    value: u128,
    knows: crate::Spawn,
}

impl Minion {
    pub fn add_one(&mut self) {
        self.value += 1;
    }
}

fn main() {
    
}

#[test]
fn creating_a_swarm() {
    let pool = Pool::new_heap(10);
    let swarm = Swarm::<Minion>::new(pool);
    assert!(swarm.max_size() == 10);
}

#[test]
fn spawning_new_swarm_instances() {
    let mut swarm = Swarm::<Minion>::new(Pool::new_heap(10));
    let spawn = swarm.spawn();
    assert!(spawn.is_ok());
    assert_eq!(swarm.count(), 1);
}

#[test]
fn referencing_spawn_instance_bodies() {
    let mut swarm = Swarm::<Minion>::new(Pool::new_heap(10));
    let spawn = swarm.spawn().unwrap();
    
    swarm.get_mut(&spawn).value = 42;
    assert_eq!(swarm.get_ref(&spawn).value, 42);
}

#[test]
fn looping_through_spawned_instances() {
    let mut swarm = Swarm::<Minion>::new(Pool::new_heap(10));
    let spawn1 = swarm.spawn().unwrap();
    let spawn2 = swarm.spawn().unwrap();
    
    assert_eq!(swarm.get_ref(&spawn1).value, 0);
    assert_eq!(swarm.get_ref(&spawn2).value, 0);

    swarm.for_each(|obj| {
        obj.value = 42;
    });

    assert_eq!(swarm.get_ref(&spawn1).value, 42);
    assert_eq!(swarm.get_ref(&spawn2).value, 42);
}

#[test]
fn destroying_spawned_instances() {
    let mut swarm = Swarm::<Minion>::new(Pool::new_heap(10));
    let spawn = swarm.spawn().unwrap();
    
    swarm.for_each(|obj| obj.value += 1);
    assert_eq!(swarm.get_ref(&spawn).value, 1);

    let copy_of_spawn = spawn.clone();
    swarm.kill(spawn);

    // After a spawn is killed, it is sill accessible but is not passed to the for loop.
    // It should not be used anymore. This is why the spawn reference is consumed by the kill
    // methode, and we had to create a copy in order to access it.
    swarm.for_each(|obj| obj.value += 1);
    assert_eq!(swarm.get_ref(&copy_of_spawn).value, 1);
    
    // If we would create a second spawn, the memory slot of the previously killed spawn is 
    // allocated to the new spawn. This override example is not how the swarm system is intended 
    // to be used, the behaviour will become unpredictable when creating and killing multiple spawns
    let spawn2 = swarm.spawn().unwrap();
    swarm.get_mut(&spawn2).value = 42;
    assert_eq!(swarm.get_ref(&copy_of_spawn).value, 42);
 }

 #[test]
fn cross_referencing_spawns_in_update_loop() {
    let mut swarm = Swarm::<Minion>::new(Pool::new_heap(10));
    let john = &swarm.spawn().unwrap();
    let cristy = &swarm.spawn().unwrap();

    let john_body = swarm.get_mut(john);
        john_body.name = String::from("John");
        john_body.knows = *cristy;

    let cristy_body = swarm.get_mut(cristy);
        cristy_body.name = String::from("Cristy");
        cristy_body.knows = *john;

    swarm::update(&mut swarm, |target, ctl| {
        let name: &str = &ctl.get_ref(target).name.clone();
        let knows: &Spawn = &ctl.get_ref(target).knows.clone();

        ctl.get_mut(knows).value = match name {
            "John" => 2, // john tells critsy to have a value of 2
            "Cristy" => 1, // cristy tell john to have a value of 1
            _ => 0,
        }
            
    });

    assert_eq!(swarm.get_ref(john).value, 1);
    assert_eq!(swarm.get_ref(cristy).value, 2);
}

// #[test]
// fn creating_spawns_during_update_loop() {
//     let mut swarm = Swarm::new(10);
//     let john = &swarm.spawn().unwrap();
//     let cristy = &swarm.spawn().unwrap();

//     let john_body = swarm.get_mut(john);
//         john_body.name = String::from("John");
//         john_body.knows = *cristy;

//     let cristy_body = swarm.get_mut(cristy);
//         cristy_body.name = String::from("Cristy");
//         cristy_body.knows = *john;

//     swarm::update(&mut swarm, |target, control| {
//         let name: &str = &control.get_ref(target).name.clone();
//         let knows: &swarm::Spawn = &control.get_ref(target).knows.clone();

//         control.get_mut(knows).value = match name {
//             "John" => 2, // john tells critsy to have a value of 2
//             "Cristy" => 1, // cristy tell john to have a value of 1
//             _ => 0,
//         }
            
//     });

//     assert_eq!(swarm.get_ref(john).value, 1);
//     assert_eq!(swarm.get_ref(cristy).value, 2);
// }