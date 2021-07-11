#[cfg(test)]

use crate::*;
use crate::{ Spawn };
//use crate as swarm;
//use std::convert::TryInto;


// sest values

pub struct SwarmData {
    counter: usize,
}

pub struct TrackSpawns {
    john: Option<Spawn>,
    cristy: Option<Spawn>,
}

#[derive(Default, Copy, Clone)]
pub struct Minion {
    name: [u8; 6],
    value: usize,
}

impl Minion {
    pub fn add_one(&mut self) {
        self.value += 1;
    }
}

fn byte_name(str: &str) -> [u8; 6] {
    let mut result = [0; 6];
    let byte_str = str.as_bytes();
    for i in 0..byte_str.len() {
        if i < 6 { result[i] = byte_str[i]; }
        else { break; }
    }
    result
}

// basic swarm tests

#[test]
fn creating_a_swarm() {
    let swarm = Swarm::<Minion, _>::new(10, ());
    assert!(swarm.max_size() == 10);
}

#[test]
fn spawning_new_swarm_instances() {
    let mut swarm = Swarm::<Minion, _>::new(10, ());
    let spawn = swarm.spawn();
    assert!(spawn.is_some());
    assert_eq!(swarm.count(), 1);
}

#[test]
fn referencing_spawn_instance_bodies() {
    let mut swarm = Swarm::<Minion, _>::new(10, ());
    let spawn = swarm.spawn().unwrap();
    
    swarm.get_mut(&spawn).value = 42;
    assert_eq!(swarm.get_ref(&spawn).value, 42);
}

#[test]
fn using_spawn_reference_info() {
    let mut swarm = Swarm::<Minion, _>::new(10, ());
    let spawn1 = swarm.spawn().unwrap();

    assert_eq!(spawn1.id(), 0);
    assert_eq!(spawn1.pos(), 0);
    assert_eq!(spawn1.active(), true);
}

#[test]
fn spawn_info_can_be_shared() {
    let mut swarm = Swarm::<Minion, _>::new(10, ());
    let spawn1 = swarm.spawn().unwrap();
    let spawn2 = spawn1.mirror();

    assert_eq!(spawn1, spawn2);
    assert_eq!(spawn1.active(), true);

    {
        let spawn3 = spawn2.mirror();
        spawn3.0.borrow_mut().active = false;
    } // spawn3 goes out of scope here!

    assert_eq!(spawn1, spawn2);
    assert_eq!(spawn1.active(), false);
}

// swarm itterator tests

#[test]
fn foreach_loop_through_spawned_instances() {
    let mut swarm = Swarm::<Minion, _>::new(10, ());
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
fn forall_loop_with_obj_references() {
    let mut swarm = Swarm::<Minion, _>::new(10, ());
    let spawn1 = swarm.spawn().unwrap();
    let spawn2 = swarm.spawn().unwrap();
    
    assert_eq!(swarm.get_ref(&spawn1).value, 0);
    assert_eq!(swarm.get_ref(&spawn2).value, 0);

    swarm.for_all(|target, list, _props| {
        list[*target].value = *target + 1;
    });

    assert_eq!(swarm.get_ref(&spawn1).value, 1);
    assert_eq!(swarm.get_ref(&spawn2).value, 2);
}

#[test]
fn forall_loop_with_swarm_properties() {
    let mut swarm = Swarm::<Minion, SwarmData>::new(10, SwarmData {
        counter: 0,
    });
    let spawn1 = swarm.spawn().unwrap();
    let spawn2 = swarm.spawn().unwrap();

    swarm.for_all(|target, list, props| {
        props.counter += 1;
        list[*target].value = props.counter;
    });

    assert_eq!(swarm.properties.counter, 2);
    assert_eq!(swarm.get_ref(&spawn1).value, 1);
    assert_eq!(swarm.get_ref(&spawn2).value, 2);
}

#[test]
fn forall_cross_referencing() {
    let mut swarm = Swarm::<Minion, TrackSpawns>::new(10, TrackSpawns {
        john: None,
        cristy: None,
    });

    let s_john = swarm.spawn().unwrap();
    let s_cristy = swarm.spawn().unwrap();

    swarm.properties.john = Some(s_john.mirror());
    swarm.properties.cristy = Some(s_cristy.mirror());

    swarm.get_mut(&s_john).name = byte_name("John");
    swarm.get_mut(&s_cristy).name = byte_name("Cristy");

    swarm.for_all(|index, list, props| {

        // john tells critsy to have a value of 2
        if list[*index].name == byte_name("John") { 
            if let Some(cristy) = &props.cristy {
                list[cristy.pos()].value = 2; 
            }
        }
        // cristy tells john to have a value of 1
        if list[*index].name == byte_name("Cristy") { 
            if let Some(john) = &props.john {
                list[john.pos()].value = 1; 
            }
        }
    });

    assert_eq!(swarm.get_ref(&s_john).value, 1);
    assert_eq!(swarm.get_ref(&s_cristy).value, 2);
}


// swarm control tests

#[test]
#[allow(unused_must_use)]
fn destroying_spawned_instances() {
    let mut swarm = Swarm::<Minion, _>::new(10, ());
    let spawn1 = swarm.spawn().unwrap();
    
    swarm.for_each(|obj| obj.value += 1);
    assert_eq!(swarm.get_ref(&spawn1).value, 1);

    swarm.kill(&spawn1);

    // After a spawn is killed, it is sill accessible but is not passed to the for loop.
    // It should not be used anymore. This is why the spawn reference is consumed by the kill
    // methode, and we had to create a copy in order to access it.
    swarm.for_each(|obj| obj.value += 1);
    assert_eq!(swarm.get_ref(&spawn1).value, 1);
    assert_eq!(spawn1.active(), false);

    // NOTE: spawn pointers that are killed, go on a re-use stack.
    // In this case spawn1 is killed and therefore nothing points to the linked data slot
    // Because we want to re-use the data slot after a kill, new spawns (in this case spawn2)
    // will points to the same data as spawn1 would have done. 
    // In this case spawn1 and spawn2 are actually the same pointer, split up by a reference counter.
    let spawn2 = swarm.spawn().unwrap();
    swarm.get_mut(&spawn2).value = 42;
    assert_eq!(swarm.get_ref(&spawn1).value, 42);
    assert_eq!(spawn1, spawn2);
 }

//  #[test]
// fn cross_referencing_spawns_in_update_loop() {
//     let mut swarm = Swarm::<Minion, SwarmData>::new(10);
//     let john = &swarm.spawn().unwrap();
//     let cristy = &swarm.spawn().unwrap();

//     let john_body = swarm.get_mut(john);
//         john_body.name = byte_name("John");
//         john_body.knows = *cristy;

//     let cristy_body = swarm.get_mut(cristy);
//         cristy_body.name = byte_name("Cristy");
//         cristy_body.knows = *john;

//     swarm.update(|target_id, swarm_ref| {
//         let name = swarm_ref.get_ref(target_id).name;

//         // john tells critsy to have a value of 2
//         if name == byte_name("John") { 
//             let cristys_id = swarm_ref.get_ref(target_id).knows;
//             swarm_ref.get_mut(&cristys_id).value = 2; 
//         }
//         // cristy tells john to have a value of 1
//         if name == byte_name("Cristy") { 
//             let johns_id = swarm_ref.get_ref(target_id).knows;
//             swarm_ref.get_mut(&johns_id).value = 1;
//         }   
//     });

//     assert_eq!(swarm.get_ref(john).value, 1);
//     assert_eq!(swarm.get_ref(cristy).value, 2);
// }

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