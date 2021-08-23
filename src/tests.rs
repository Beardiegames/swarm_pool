//! Swarm unit tests

#[cfg(test)]
use crate::*;
use crate::{ Spawn };
//use crate::tools::byte_str::ByteStr;

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
    name: &'static str,
    value: usize,
}

impl Minion {
    pub fn add_one(&mut self) {
        self.value += 1;
    }
}

// basic swarm tests

#[test]
fn creating_a_swarm() {
    let swarm = Swarm::<Minion, _>::new(10, ());
    assert!(swarm.capacity() == 10);
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
    
    swarm.fetch(&spawn).value = 42;
    assert_eq!(swarm.fetch_ref(&spawn).value, 42);
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
    
    assert_eq!(swarm.fetch_ref(&spawn1).value, 0);
    assert_eq!(swarm.fetch_ref(&spawn2).value, 0);

    swarm.for_each(|obj| {
        obj.value = 42;
    });

    assert_eq!(swarm.fetch_ref(&spawn1).value, 42);
    assert_eq!(swarm.fetch_ref(&spawn2).value, 42);
}

#[test]
fn forall_loop_with_obj_references() {
    let mut swarm = Swarm::<Minion, _>::new(10, ());
    let spawn1 = swarm.spawn().unwrap();
    let spawn2 = swarm.spawn().unwrap();
    
    assert_eq!(swarm.fetch_ref(&spawn1).value, 0);
    assert_eq!(swarm.fetch_ref(&spawn2).value, 0);

    swarm.for_all(|fetch, list, _props| {
        list[*fetch].value = *fetch + 1;
    });

    assert_eq!(swarm.fetch_ref(&spawn1).value, 1);
    assert_eq!(swarm.fetch_ref(&spawn2).value, 2);
}

#[test]
fn forall_loop_with_swarm_properties() {
    let mut swarm = Swarm::<Minion, SwarmData>::new(10, SwarmData {
        counter: 0,
    });
    let spawn1 = swarm.spawn().unwrap();
    let spawn2 = swarm.spawn().unwrap();

    swarm.for_all(|fetch, list, props| {
        props.counter += 1;
        list[*fetch].value = props.counter;
    });

    assert_eq!(swarm.properties.counter, 2);
    assert_eq!(swarm.fetch_ref(&spawn1).value, 1);
    assert_eq!(swarm.fetch_ref(&spawn2).value, 2);
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

    swarm.fetch(&s_john).name = "John";
    swarm.fetch(&s_cristy).name = "Cristy";

    swarm.for_all(|index, list, props| {

        // john tells critsy to have a value of 2
        if list[*index].name == "John" { 
            if let Some(cristy) = &props.cristy {
                list[cristy.pos()].value = 2; 
            }
        }
        // cristy tells john to have a value of 1
        if list[*index].name == "Cristy" { 
            if let Some(john) = &props.john {
                list[john.pos()].value = 1; 
            }
        }
    });

    assert_eq!(swarm.fetch_ref(&s_john).value, 1);
    assert_eq!(swarm.fetch_ref(&s_cristy).value, 2);
}


// swarm control tests

#[test]
#[allow(unused_must_use)]
fn killing_spawned_instances() {
    let mut swarm = Swarm::<Minion, _>::new(10, ());
    let spawn1 = swarm.spawn().unwrap();
    
    swarm.for_each(|obj| obj.value += 1);
    assert_eq!(swarm.fetch_ref(&spawn1).value, 1);

    swarm.kill(&spawn1);
    assert_eq!(swarm.len, 0);

    // After a spawn is killed, it is sill accessible but is not passed to the for loop.
    // It should not be used anymore. This is why the spawn reference is consumed by the kill
    // methode, and we had to create a copy in order to access it.
    swarm.for_each(|obj| obj.value += 1);
    assert_eq!(swarm.fetch_ref(&spawn1).value, 1);
    assert_eq!(spawn1.active(), false);

    swarm.for_all(|tar, list, _props| list[*tar].value += 1);
    assert_eq!(swarm.fetch_ref(&spawn1).value, 1);
    assert_eq!(spawn1.active(), false);

    swarm.update(|ctx| ctx.target().value += 1);
    assert_eq!(swarm.fetch_ref(&spawn1).value, 1);
    assert_eq!(spawn1.active(), false);

    // NOTE: spawn pointers that are killed, go on a re-use stack.
    // In this case spawn1 is killed and therefore nothing points to the linked data slot
    // Because we want to re-use the data slot after a kill, new spawns (in this case spawn2)
    // will points to the same data as spawn1 would have done. 
    // In this case spawn1 and spawn2 are actually the same pointer, split up by a reference counter.
    let spawn2 = swarm.spawn().unwrap();
    swarm.fetch(&spawn2).value = 42;
    assert_eq!(swarm.fetch_ref(&spawn1).value, 42);
    assert_eq!(spawn1, spawn2);
 }

 #[test]
 fn kill_all_spawned_instances() {
    let mut swarm = Swarm::<Minion, _>::new(10, ());
    
    let spawn1 = swarm.spawn().unwrap();
    let spawn2 = swarm.spawn().unwrap();
    let spawn3 = swarm.spawn().unwrap();
    assert_eq!(swarm.len, 3);
    assert_eq!(spawn1.active(), true);
    assert_eq!(spawn2.active(), true);
    assert_eq!(spawn3.active(), true);
    
    swarm.for_each(|obj| obj.value += 1);
    assert_eq!(swarm.fetch_ref(&spawn1).value, 1);

    swarm.kill_all();
    assert_eq!(swarm.len, 0);

    // After a spawn is killed, it is sill accessible but is not passed to the for loop.
    // It should not be used anymore.
    swarm.for_each(|obj| obj.value += 1);
    assert_eq!(swarm.fetch_ref(&spawn1).value, 1);
    assert_eq!(swarm.fetch_ref(&spawn2).value, 1);
    assert_eq!(swarm.fetch_ref(&spawn3).value, 1);
    assert_eq!(spawn1.active(), false);
    assert_eq!(spawn2.active(), false);
    assert_eq!(spawn3.active(), false);

    swarm.for_all(|tar, list, _props| list[*tar].value += 1);
    assert_eq!(swarm.fetch_ref(&spawn1).value, 1);
    assert_eq!(swarm.fetch_ref(&spawn2).value, 1);
    assert_eq!(swarm.fetch_ref(&spawn3).value, 1);
    assert_eq!(spawn1.active(), false);
    assert_eq!(spawn2.active(), false);
    assert_eq!(spawn3.active(), false);

    swarm.update(|ctx| ctx.target().value += 1);
    assert_eq!(swarm.fetch_ref(&spawn1).value, 1);
    assert_eq!(swarm.fetch_ref(&spawn2).value, 1);
    assert_eq!(swarm.fetch_ref(&spawn3).value, 1);
    assert_eq!(spawn1.active(), false);
    assert_eq!(spawn2.active(), false);
    assert_eq!(spawn3.active(), false);

    // NOTE: spawn pointers that are killed, go on a re-use stack.
    // In this case spawn1 is killed and therefore nothing points to the linked data slot
    // Because we want to re-use the data slot after a kill, new spawns (in this case spawn2)
    // will points to the same data as spawn1 would have done. 
    // In this case spawn1 and spawn2 are actually the same pointer, split up by a reference counter.
    let spawn4 = swarm.spawn().unwrap();
    swarm.fetch(&spawn4).value = 42;
    assert_eq!(swarm.fetch_ref(&spawn1).value, 42);
    assert_eq!(spawn1, spawn4);
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

    swarm.fetch(&s_john).name = "John";
    swarm.fetch(&s_cristy).name = "Cristy";

    swarm.update(|ctx| {
        let name = ctx.target().name;
        let cristy = ctx.properties.cristy.as_ref().unwrap().mirror();
        let john = ctx.properties.john.as_ref().unwrap().mirror();

        // john tells critsy to have a value of 2
        if name == "John" { 
            ctx.fetch(&cristy).value = 2; 
        }
        // cristy tells john to have a value of 1
        if name == "Cristy" { 
            ctx.fetch(&john).value = 1; 
        }
    });

    assert_eq!(swarm.fetch_ref(&s_john).value, 1);
    assert_eq!(swarm.fetch_ref(&s_cristy).value, 2);
}

#[test]
fn creating_spawns_during_update_loop() {
    let mut swarm = Swarm::<Minion, _>::new(10, ());
    
    let _spawn1 = &swarm.spawn().unwrap();
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
    //     println!("PRE-KILL: {:?}", ctx.fetch_spawn());
    // });
    swarm.update(|ctx| {
        if ctx.target_spawn().id() == 2 { ctx.kill(&ctx.fetch_spawn(&0)); }
    });
    // swarm.update(|ctx| {
    //     println!("POST-KILL: {:?}", ctx.fetch_spawn());
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

#[derive(Default, Clone, Debug, PartialEq)] struct Image(bool);
#[derive(Default, Clone, Debug, PartialEq)] struct Position(f32, f32);
#[derive(Default, Clone, Debug, PartialEq)] struct Speed(f32);

#[derive(Default, Clone, Debug)]
pub struct Entity {
    name: &'static str,
    
    image_component: Option<Image>,
    position_component: Option<Position>,
    speed_component: Option<Speed>,
}


#[test]
fn using_swarm_for_ECS() {
    let mut swarm = Swarm::<Entity, _>::new(10, ());
    
    let building = swarm.spawn().unwrap();
    {
        let entity = swarm.fetch(&building);
            entity.name = "Sixteenth Chapel";
            entity.image_component = Some(Image(false));
            entity.position_component = Some(Position(3.0, 5.0));
            entity.speed_component = None;
    }

    let truck = swarm.spawn().unwrap();
    {
        let entity = swarm.fetch(&truck);
            entity.name = "Cargo truck";
            entity.image_component = Some(Image(false));
            entity.position_component = Some(Position(8.0, 6.0));
            entity.speed_component = Some(Speed(1.0));
    }

    assert_eq!(swarm.fetch_ref(&building).position_component, Some(Position(3.0, 5.0)));
    assert_eq!(swarm.fetch_ref(&building).image_component, Some(Image(false)));

    assert_eq!(swarm.fetch_ref(&truck).position_component, Some(Position(8.0, 6.0)));
    assert_eq!(swarm.fetch_ref(&truck).image_component, Some(Image(false)));

    // # MOVE SYSTEM
    swarm.for_all(|tar, pool, props|{
        if let ( 
            Some(position_component), 
            Some(speed_component)
        ) = (
            &mut pool[*tar].position_component, 
            &mut pool[*tar].speed_component 
        ){
            position_component.0 += speed_component.0;
        }
    });

    // Only the truck moves from position 8.0 to 9.0, the building does not 
    // move because it does not have a Speed component which is required 
    // by the move system.
    assert_eq!(swarm.fetch_ref(&building).position_component, Some(Position(3.0, 5.0)));
    assert_eq!(swarm.fetch_ref(&truck).position_component, Some(Position(9.0, 6.0)));

    // # DRAW SYSTEM
    swarm.for_all(|tar, pool, props|{
        if let ( 
            Some(position_component), 
            Some(image_component)
        ) = (
            &mut pool[*tar].position_component, 
            &mut pool[*tar].image_component 
        ){
            image_component.0 = true;
        }
    });

    // Both building and truck should be updated because both have a Position 
    // and an Image component, which are required by the draw system.
    assert_eq!(swarm.fetch_ref(&building).image_component, Some(Image(true)));
    assert_eq!(swarm.fetch_ref(&truck).image_component, Some(Image(true)));
}