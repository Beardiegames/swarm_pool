
#[cfg(test)]
use std::time::SystemTime;
#[allow(unused_imports)]
use super::*;


pub struct Minion { pub times_called: u128 }
impl Default for Minion { 
    fn default() -> Self { Minion { times_called: 0 }}
}


#[test]
fn swarms_can_be_constructed() {
    let swarm = Swarm::<Minion>::new(10);
    assert!(swarm.spawns.len() == 0);
}

#[test]
fn minions_can_be_spawned() {
    let mut swarm = Swarm::<Minion>::new(10);
    assert!(swarm.spawns.len() == 0);

    let mut spawn = swarm.spawn().unwrap();
    assert!(
        spawn.clone().0 == 9, 
        "first spawn should be '9' but is '{}' instead!", 
        spawn.0
    );
    assert!(swarm.spawns.len() == 1);
}

#[test]
fn entities_are_data_which_spawns_refer_to() {
    let mut swarm = Swarm::<Minion>::new(10);

    if let Some(spawn) = swarm.spawn() {
        assert!(swarm.get_body(&spawn).times_called == 0);
    } else {
        assert!(false, "Spawn Failed!");
    }
}

#[test]
fn spawns_can_be_killed() {
    let mut swarm = Swarm::<Minion>::new(10);

    if let Some(spawn) = swarm.spawn() {
        swarm.kill(spawn);
    } else {
        assert!(false, "Spawn Failed!");
    }
    assert!(swarm.spawns.len() == 0);
}

// #[test]
// fn spawns_can_signup_for_behaviours() {
//     let mut swarm = Swarm::<Minion>::new(10);
//     let behaviour = swarm.behaviour(|mut obj| { 
//         obj.times_called += 1; 
//     });
    
//     if let Some(spawn) = swarm.spawn() {
//         swarm.assign(&behaviour, &spawn);
//         assert!(swarm.is_assigned(&behaviour, &spawn));
//     } else {
//         assert!(false, "Spawn Failed!");
//     }  
// }


// #[test]
// fn a_spawns_aspects_can_be_called() {
//     let mut swarm = Swarm::<Minion>::new(10);
//     let make_web = swarm.define_aspect(|mut obj| { obj.times_called += 1; });
//     let spider = swarm.create_species()
//         .add_aspect(&make_web, true)
//         .id();

//     if let Some(spawn) = swarm.spawn(&spider) {
//         swarm.kill(spawn);
//     } else {
//         assert!(false, "Spawn Failed!");
//     }
// }

// #[test]
// fn natures_are_triggered_for_all_assigned_spawns() {
//     assert!(false, "no test code implemented");
// }

// #[test]
// fn aspects_can_trigger_another_spawns_aspects() {
//     assert!(false, "no test code implemented");
// }

#[test]
fn update_speed() {
    let mut swarm = Swarm::<Minion>::new(10);
    let spawn = swarm.spawn().unwrap();

    let now = SystemTime::now();
    for _i in 0..100_000_000 { 
        swarm.for_each(some_system);
    }
    let elapsed_res = now.elapsed();

    match elapsed_res {
        Ok(elapsed) => assert!(false, "updates {} Mil calls/s", 
            swarm.get_body(&spawn).times_called as f64 * (1_000.0 / elapsed.as_nanos() as f64)
        ),
        Err(e)      => assert!(false, "Error: {:?}", e),
    }
}


pub trait AddOne {
    fn add_one(&mut self);
}
impl AddOne for Minion {
    fn add_one(&mut self) { self.times_called += 1 }
}

pub fn some_system(spawn: &Spawn, pool: &mut Pool<Minion>) {
    pool.get_body(spawn).add_one();
}
