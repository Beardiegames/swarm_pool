
#[cfg(test)]
use std::time::SystemTime;
#[allow(unused_imports)]
use super::*;

#[derive(Default, Copy, Clone)]
pub struct Minion { pub times_called: u128 }


// #[test]
// fn swarms_can_be_constructed() {
//     let swarm = Swarm::<Minion>::new(10);
//     assert!(swarm.control.spawns.len() == 0);
// }

// #[test]
// fn minions_can_be_spawned() {
//     let mut swarm = Swarm::<Minion>::new(10);
//     assert!(swarm.control.spawns.len() == 0);

//     let mut spawn = swarm.control.spawn().unwrap();
//     assert!(
//         spawn.clone().0 == 9, 
//         "first spawn should be '9' but is '{}' instead!", 
//         spawn.0
//     );
//     assert!(swarm.control.spawns.len() == 1);
// }

// #[test]
// fn entities_are_data_which_spawns_refer_to() {
//     let mut swarm = Swarm::<Minion>::new(10);

//     if let Some(spawn) = swarm.control.spawn() {
//         assert!(swarm.pool.get_ref(&spawn).times_called == 0);
//     } else {
//         assert!(false, "Spawn Failed!");
//     }
// }

// #[test]
// fn spawns_can_be_killed() {
//     let mut swarm = Swarm::<Minion>::new(10);

//     if let Some(spawn) = swarm.control.spawn() {
//         swarm.control.kill(&spawn);
//     } else {
//         assert!(false, "Spawn Failed!");
//     }
//     assert!(swarm.control.spawns.len() == 0);
// }

#[test]
fn single_update_speed() {
    let mut swarm = Swarm::<Minion>::new();
    //let spawn = swarm.pool.spawn().unwrap();
    //swarm.pool.spawn();


    // get thread base speed
    let mut baseline = [Minion::default()];
    let now = SystemTime::now();
    for _i in 0..100_000_000 {
        for j in 0..1 {
            baseline[j].add_one();
        }
    }
    let elapsed_base = now.elapsed();


    // get thread vec speed
    let mut vec_test = vec![Minion::default()];
    let now = SystemTime::now();
    for _i in 0..100_000_000 {
        for j in 0..1 {
            vec_test[j].add_one();
        }
    }
    let elapsed_vec = now.elapsed();


    // get swarm system speed
    //let mut ref_test: *mut Minion = &mut vec![Minion::default()][0];
    //let mut ref_vals = [ref_test];
    let mut ref_arr: Swarm<Minion> = Swarm::new();
    let now = SystemTime::now();
    for _i2 in 0..100_000_000 { 
        for_each(&mut ref_arr, add_system);
    }
    let elapsed_res = now.elapsed();


    let base_time = (elapsed_base.unwrap().as_nanos() as f64) * 0.001;
    let base_speed =  baseline[0].times_called as f64 / base_time;
    println!("stack baseline was called {} times", baseline[0].times_called);
    println!("stack max speed = {} Mil calls/s\n", base_speed);

    let vec_time = (elapsed_vec.unwrap().as_nanos() as f64) * 0.001;
    let vec_speed =  vec_test[0].times_called as f64 / vec_time;
    println!("heap baseline was called {} times", vec_test[0].times_called);
    println!("heap max speed = {} Mil calls/s\n", vec_speed);

    let swarm_time = (elapsed_res.unwrap().as_nanos() as f64) * 0.001;
    let swarm_speed = ref_arr.dat[0].times_called as f64 / swarm_time;
    println!("spawn was called {} times", ref_arr.dat[0].times_called);
    println!("system updates = {} Mil calls/s\n", swarm_speed);

    let weight = ((swarm_speed / base_speed) * 100_000.0).round() / 10_000.0;
    println!("running at '{}%' of absolute maximum", weight);

    // unsafe { 
    //     let swarm_time = (elapsed_res.unwrap().as_nanos() as f64) * 0.001;
    //     let swarm_speed =  (*ref_vals[0]).times_called as f64 / swarm_time;
    //     println!("spawn was called {} times", (*ref_vals[0]).times_called);
    //     println!("system updates = {} Mil calls/s\n", swarm_speed);

    //     let weight = ((swarm_speed / base_speed) * 100_000.0).round() / 10_000.0;
    //     println!("running at '{}%' of absolute maximum", weight);
    // }

    
    //assert!(weight >= 20.0, "Goal should be at least 20% of the maximum speed (Maxixmum is the durations of single methode call)");
    assert!(false);
}

fn add_system(x: &mut Minion) {
    x.add_one();
}


pub trait AddOne {
    fn add_one(&mut self);
}
impl AddOne for Minion {
    fn add_one(&mut self) { self.times_called += 1 }
}

// pub fn some_system(spawn: &Spawn, pool: &mut Pool<Minion>){ //}, ctl: &mut SwarmControl) {
//     pool.get_mut(spawn).add_one();
//     //ctl.kill(spawn);
// }


// impl Swarm<Minion> {
//     pub fn for_each(&mut self) { //}, &mut SwarmControl)) {
//         for i in 0..self.spawns.count {
//             self.pool.get_mut(&self.spawns.active[i]).add_one();
//         }
//     }
// }
