
#[cfg(test)]
use std::time::SystemTime;
#[allow(unused_imports)]
use super::*;

struct Minion { pub times_called: u128 }
impl Default for Minion { 
    fn default() -> Self { Minion { times_called: 0 }}
}

#[test]
fn swarms_can_be_constructed() {
    let swarm = Swarm::<Minion>::new(10);
    assert!(swarm.spawned_minions.len() == 0);
}

#[test]
fn swarms_accept_new_behaviours() {
    let mut swarm = Swarm::<Minion>::new(10);
    swarm.behaviour(|mut minion| { minion.times_called += 1; });

    assert!(swarm.behaviours.len() == 1); 
}

#[test]
fn minions_can_be_spawned() {
    let mut swarm = Swarm::<Minion>::new(10);
    let spawn = swarm.spawn();
    assert!(spawn.is_some());
    assert!(swarm.spawned_minions.len() == 1);
}

#[test]
fn minions_are_data_which_spawns_refer_to() {
    let mut swarm = Swarm::<Minion>::new(10);

    if let Some(spawn) = swarm.spawn() {
        assert!(swarm.minion(&spawn).times_called == 0);
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
    assert!(swarm.spawned_minions.len() == 0);
}

#[test]
fn spawns_can_signup_for_behaviours() {
    let mut swarm = Swarm::<Minion>::new(10);
    let behaviour = swarm.behaviour(|mut minion| { minion.times_called += 1; });
    
    if let Some(spawn) = swarm.spawn() {
        swarm.assign(&behaviour, &spawn);
        assert!(swarm.is_assigned(&behaviour, &spawn));
    } else {
        assert!(false, "Spawn Failed!");
    }  
}

#[test]
fn behaviours_can_be_triggered_for_all_assigned_spawns() {
    let mut swarm = Swarm::<Minion>::new(10);
    let behaviour = swarm.behaviour(|mut minion| { minion.times_called += 1; });
    
    if let Some(spawn_a) = swarm.spawn() {
        swarm.assign(&behaviour, &spawn_a);
        if let Some(spawn_b) = swarm.spawn() {
            swarm.assign(&behaviour, &spawn_b);
            swarm.trigger_all(&behaviour);
            assert!(swarm.minion(&spawn_a).times_called == 1);
            assert!(swarm.minion(&spawn_b).times_called == 1);
        } else {
            assert!(false, "Spawn Failed!");
        } 
    } else {
        assert!(false, "Spawn Failed!");
    } 
}

#[test]
fn behaviours_can_be_triggered_for_a_single_spawns() {
    let mut swarm = Swarm::<Minion>::new(10);
    let behaviour = swarm.behaviour(|mut minion| { minion.times_called += 1; });
    
    if let Some(spawn_a) = swarm.spawn() {
        swarm.assign(&behaviour, &spawn_a);
        if let Some(spawn_b) = swarm.spawn() {
            swarm.assign(&behaviour, &spawn_b);
            swarm.trigger(&behaviour, &spawn_b);
            assert!(swarm.minion(&spawn_a).times_called == 0);
            assert!(swarm.minion(&spawn_b).times_called == 1);
        } else {
            assert!(false, "Spawn Failed!");
        } 
    } else {
        assert!(false, "Spawn Failed!");
    } 
}


#[test]
fn behaviours_can_be_trigger_other_behaviours() {
    let mut swarm = Swarm::<Minion>::new(10);
    let spawn_a = swarm.spawn().unwrap();
    let spawn_b = swarm.spawn().unwrap();

    let behaviour = swarm.behaviour(|mut minion| { 
        minion.times_called += 1; 
    });

    swarm.assign(&behaviour, &spawn_a);
    swarm.assign(&behaviour, &spawn_b);
}

#[test]
fn update_speed() {
    let mut swarm = Swarm::<Minion>::new(10);
    let behaviour = swarm.behaviour(|mut minion| { minion.times_called += 1; });
    let spawn = swarm.spawn().unwrap();

    swarm.assign(&behaviour, &spawn);

    let now = SystemTime::now();
    for _i in 0..100_000_000 { swarm.trigger_all(&behaviour); }
    let elapsed_res = now.elapsed();

    match elapsed_res {
        Ok(elapsed) => assert!(false, "updates {} Mil calls/s", 
            swarm.minion(&spawn).times_called as f64 * (1_000.0 / elapsed.as_nanos() as f64)
        ),
        Err(e)      => assert!(false, "Error: {:?}", e),
    }

}
