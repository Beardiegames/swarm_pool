#[cfg(test)]

use swarm::{ Swarm, SwarmError, Spawn };

#[derive(Default, Copy, Clone)]
pub struct Minion {
    times_summoned: u128,
}

impl Minion {
    pub fn add_one(&mut self) {
        self.times_summoned += 1;
    }
}

fn main() {
    
}

#[test]
fn creating_a_swarm() {
    let swarm = Swarm::<Minion>::new(10);
    assert!(swarm.max_size() == 10);
}

#[test]
fn spawning_swarm_entity_instances() {
    let mut swarm = Swarm::<Minion>::new(10);
    let spawn = swarm.spawn();
    assert!(spawn.is_some());
    assert_eq!(swarm.count(), 1);
}

#[test]
fn using_spawned_instances() {
    let mut swarm = Swarm::<Minion>::new(10);
    let spawn = swarm.spawn().unwrap();
    
    swarm.get_mut(&spawn).times_summoned = 42;
    assert_eq!(swarm.get_ref(&spawn).times_summoned, 42);
}

#[test]
fn looping_through_spawned_instances() {
    let mut swarm = Swarm::<Minion>::new(10);
    let spawn1 = swarm.spawn().unwrap();
    let spawn2 = swarm.spawn().unwrap();
    
    assert_eq!(swarm.get_ref(&spawn1).times_summoned, 0);
    assert_eq!(swarm.get_ref(&spawn2).times_summoned, 0);

    swarm.for_each(|obj| {
        obj.times_summoned = 42;
    });

    assert_eq!(swarm.get_ref(&spawn1).times_summoned, 42);
    assert_eq!(swarm.get_ref(&spawn2).times_summoned, 42);
}

#[test]
fn destroying_spawned_instances() {
    let mut swarm = Swarm::<Minion>::new(10);
    let spawn = swarm.spawn().unwrap();
    
    swarm.for_each(|obj| obj.times_summoned += 1);
    assert_eq!(swarm.get_ref(&spawn).times_summoned, 1);

    let copy_of_spawn = spawn.clone();
    swarm.kill(spawn);

    // After a spawn is killed, it is sill accessible but is not passed to the for loop.
    // It should not be used anymore. This is why the spawn reference is consumed by the kill
    // methode, and we had to create a copy in order to access it.
    swarm.for_each(|obj| obj.times_summoned += 1);
    assert_eq!(swarm.get_ref(&copy_of_spawn).times_summoned, 1);
    
    // If we would create a second spawn, the memory slot of the previously killed spawn is 
    // allocated to the new spawn. This override example is not how the swarm system is intended 
    // to be used, the behaviour will become unpredictable when creating and killing multiple spawns
    let spawn2 = swarm.spawn().unwrap();
    swarm.get_mut(&spawn2).times_summoned = 42;
    assert_eq!(swarm.get_ref(&copy_of_spawn).times_summoned, 42);
 }