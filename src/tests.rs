#[cfg(test)]

use crate::*;
use crate::{ Spawn };
use crate as swarm;
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
    
    swarm.get(&spawn).value = 42;
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

    swarm.get(&s_john).name = byte_name("John");
    swarm.get(&s_cristy).name = byte_name("Cristy");

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
fn killing_spawned_instances() {
    let mut swarm = Swarm::<Minion, _>::new(10, ());
    let spawn1 = swarm.spawn().unwrap();
    
    swarm.for_each(|obj| obj.value += 1);
    assert_eq!(swarm.get_ref(&spawn1).value, 1);

    swarm.kill(&spawn1);
    assert_eq!(swarm.len, 0);

    // After a spawn is killed, it is sill accessible but is not passed to the for loop.
    // It should not be used anymore. This is why the spawn reference is consumed by the kill
    // methode, and we had to create a copy in order to access it.
    swarm.for_each(|obj| obj.value += 1);
    assert_eq!(swarm.get_ref(&spawn1).value, 1);
    assert_eq!(spawn1.active(), false);

    swarm.for_all(|tar, list, _props| list[*tar].value += 1);
    assert_eq!(swarm.get_ref(&spawn1).value, 1);
    assert_eq!(spawn1.active(), false);

    swarm.update(|ctx| ctx.target().value += 1);
    assert_eq!(swarm.get_ref(&spawn1).value, 1);
    assert_eq!(spawn1.active(), false);

    // NOTE: spawn pointers that are killed, go on a re-use stack.
    // In this case spawn1 is killed and therefore nothing points to the linked data slot
    // Because we want to re-use the data slot after a kill, new spawns (in this case spawn2)
    // will points to the same data as spawn1 would have done. 
    // In this case spawn1 and spawn2 are actually the same pointer, split up by a reference counter.
    let spawn2 = swarm.spawn().unwrap();
    swarm.get(&spawn2).value = 42;
    assert_eq!(swarm.get_ref(&spawn1).value, 42);
    assert_eq!(spawn1, spawn2);
 }

 #[test]
fn update_cross_referencing() {
    let mut swarm = Swarm::<Minion, TrackSpawns>::new(10, TrackSpawns {
        john: None,
        cristy: None,
    });

    let s_john = swarm.spawn().unwrap();
    let s_cristy = swarm.spawn().unwrap();

    swarm.properties.john = Some(s_john.mirror());
    swarm.properties.cristy = Some(s_cristy.mirror());

    swarm.get(&s_john).name = byte_name("John");
    swarm.get(&s_cristy).name = byte_name("Cristy");

    swarm.update(|ctx| {
        let name = ctx.target().name;
        let cristy = ctx.properties.cristy.as_ref().unwrap().mirror();
        let john = ctx.properties.john.as_ref().unwrap().mirror();

        // john tells critsy to have a value of 2
        if name == byte_name("John") { 
            ctx.get(&cristy.pos()).value = 2; 
        }
        // cristy tells john to have a value of 1
        if name == byte_name("Cristy") { 
            ctx.get(&john.pos()).value = 1; 
        }
    });

    assert_eq!(swarm.get_ref(&s_john).value, 1);
    assert_eq!(swarm.get_ref(&s_cristy).value, 2);
}

#[test]
fn creating_spawns_during_update_loop() {
    let mut swarm = Swarm::<Minion, _>::new(10, ());
    
    let spawn1 = &swarm.spawn().unwrap();
    assert_eq!(swarm.count(), 1);

    swarm.update(|ctx| {
        if ctx.head() == 0 { ctx.spawn();} 
    });
    assert_eq!(swarm.count(), 2);

    swarm.update(|ctx| {
        if ctx.head() <= 1 { ctx.spawn();}
    });
    assert_eq!(swarm.count(), 4);

    swarm.update(|ctx| {
        if ctx.head() <= 3 { ctx.spawn();} 
    });
    assert_eq!(swarm.count(), 8);
}

#[test]
fn killing_spawns_during_update_loop() {
    let mut swarm = Swarm::<Minion, _>::new(10, ());
    
    let spawn1 = &swarm.spawn().unwrap();
    let spawn2 = &swarm.spawn().unwrap();
    let spawn3 = &swarm.spawn().unwrap();
    assert_eq!(swarm.len, 3);
    assert_eq!(spawn1.pos(), 0);
    assert_eq!(spawn2.pos(), 1);
    assert_eq!(spawn3.pos(), 2);

    // kill spawn2 on pos index 1
    swarm.update(|ctx| {
        let spawn = ctx.target_spawn();
        if spawn.id() == 1 { ctx.kill_current(); }
        println!("CTX len={}", ctx.len);
    });
    assert_eq!(swarm.count(), 2);

    assert_eq!(spawn1.pos(), 0);
    assert_eq!(spawn2.pos(), 2);
    assert_eq!(spawn3.pos(), 1);
    assert_eq!(spawn1.active(), true);
    assert_eq!(spawn2.active(), false);
    assert_eq!(spawn3.active(), true);

    // spawn new
    let spawn4 = &swarm.spawn().unwrap();
    assert_eq!(spawn4, spawn2, "spawn4 should be the same as spawn2, because spawn2 was freed after kill");
    assert_eq!(swarm.count(), 3);

    assert_eq!(spawn1.pos(), 0);
    assert_eq!(spawn2.pos(), 2);
    assert_eq!(spawn3.pos(), 1);
    assert_eq!(spawn4.pos(), 2);
    assert_eq!(spawn1.active(), true);
    assert_eq!(spawn2.active(), true);
    assert_eq!(spawn3.active(), true);
    assert_eq!(spawn4.active(), true);

    // kill spawn1 on pos index 2
    // swarm.update(|ctx| {
    //     println!("PRE-KILL: {:?}", ctx.target_spawn());
    // });
    swarm.update(|ctx| {
        if ctx.target_spawn().id() == 2 { ctx.kill(&ctx.spawn_at(&0)); }
    });
    // swarm.update(|ctx| {
    //     println!("POST-KILL: {:?}", ctx.target_spawn());
    // });
    assert_eq!(swarm.count(), 2);

    assert_eq!(spawn1.pos(), 2);
    assert_eq!(spawn2.pos(), 0);
    assert_eq!(spawn3.pos(), 1);
    assert_eq!(spawn1.active(), false);
    assert_eq!(spawn2.active(), true);
    assert_eq!(spawn3.active(), true);

    // kill all spawns
    swarm.update(|ctx| {
        println!("PRE-KILL: {:?}", ctx.target_spawn());
        ctx.kill_current();
        println!("POST-KILL: {:?}", ctx.target_spawn());
    });
    assert_eq!(swarm.count(), 0);

    assert_eq!(spawn1.pos(), 2);
    assert_eq!(spawn2.pos(), 1);
    assert_eq!(spawn3.pos(), 0);
    assert_eq!(spawn1.active(), false);
    assert_eq!(spawn2.active(), false);
    assert_eq!(spawn3.active(), false);
}