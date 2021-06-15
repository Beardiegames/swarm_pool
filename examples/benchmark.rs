
#[allow(unused_imports)]
use std::cmp::Ordering;
use swarm::{ Swarm, Spawn };
use swarm::ecs::{ Entity, Component, RequiredComponents, System };

#[derive(Default, Copy, Clone)]
pub struct Minion {
    times_summoned: u128,
}

impl Minion {
    pub fn add_one(&mut self) {
        self.times_summoned += 1;
    }
}

pub struct AddSystem;

impl System<Minion> for AddSystem {

    fn update(&mut self, spawn: &Spawn, swarm: &mut Swarm<Minion>) {
        swarm.get_mut(spawn).times_summoned += 1;
    }
}

fn main() {
    let mut run_id: usize = 0;

    let (vec_bn1, for_bn1, ecs_bn1) = bench_with_objects(&mut run_id);
    let (vec_bn2, for_bn2, ecs_bn2) = bench_with_objects(&mut run_id);
    let (vec_bn3, for_bn3, ecs_bn3) = bench_with_objects(&mut run_id);

    let bvec = vec_bn1.merge(vec_bn2).merge(vec_bn3);
    let bfor = for_bn1.merge(for_bn2).merge(for_bn3);
    let becs = ecs_bn1.merge(ecs_bn2).merge(ecs_bn3);

    println!("# RESULTS TOTAL:");

    println!("Plain vec results:");
    println!(" - average of '{}M' calls/s", bvec.avg().round());

    let vmin = bvec.min();
    println!(" - lowest of '{}M' calls/s -> bench #{}", 
        vmin.1.round(), vmin.0);
    let vmax = bvec.max();
    println!(" - highest of '{}M' calls/s -> becnh #{}", 
        vmax.1.round(), vmax.0);
    
    println!("Swarm foreach results:");
    println!(" - average of '{}M' calls/s", bfor.avg().round());
    println!(" - av. speed was '{}%' of plain vector speed", 
        ((bfor.avg() / bvec.avg()) * 100_000.0).round() / 1_000.0);

    let vmin = bfor.min();
    println!(" - lowest of '{}M' calls/s -> bench #{}", 
        vmin.1.round(), vmin.0);
    let vmax = bfor.max();
    println!(" - highest of '{}M' calls/s -> bench #{}", 
        vmax.1.round(), vmax.0);
    

    println!("Swarm ecs results:");
    println!(" - average of '{}M' calls/s", becs.avg().round());
    println!(" - average speed was '{}%' of plain vector speed", 
        ((becs.avg() / bvec.avg()) * 100_000.0).round() / 1_000.0);

    let vmin = becs.min();
    println!(" - lowest of '{}M' calls/s -> becnh #{}", 
        vmin.1.round(), vmin.0);
    let vmax = becs.max();
    println!(" - highest of '{}M' calls/s -> becnh #{}", 
        vmax.1.round(), vmax.0);
    
}

fn bench_with_objects(run_id: &mut usize) -> (Bench, Bench, Bench) {
    
    let (v_spd1, f_spd1, e_spd1) = bench_with(run_id, 1);
    let (v_spd2, f_spd2, e_spd2) = bench_with(run_id, 10);
    let (v_spd2, f_spd2, e_spd2) = bench_with(run_id, 100);
    let (v_spd3, f_spd3, e_spd3) = bench_with(run_id, 1_000);
    let (v_spd4, f_spd4, e_spd4) = bench_with(run_id, 10_000);
    let (v_spd5, f_spd5, e_spd5) = bench_with(run_id, 100_000);
    let (v_spd6, f_spd6, e_spd6) = bench_with(run_id, 1_000_000);

    println!("--");
    (   
        Bench (vec![v_spd1, v_spd2, v_spd3, v_spd4, v_spd5, v_spd6]),
        Bench (vec![f_spd1, f_spd2, f_spd3, f_spd4, f_spd5, f_spd6]),
        Bench (vec![e_spd1, e_spd2, e_spd3, e_spd4, e_spd5, e_spd6]),
    )
}

fn bench_with(run_id: &mut usize, objects: usize) -> ((usize, f64), (usize, f64), (usize, f64)) {
    let fn_avg = |x: f64, vec: f64| (100.0 * x / vec).round();

    std::thread::sleep(std::time::Duration::from_millis(500));
    //println!("START BENCH over {} objects using {} test cycles", objects, cycles);
    
    let vec_spd = vec_bencher(run_id, objects);
    println!("> '{}M' call/s", vec_spd.1.round());

    let for_spd = for_bencher(run_id, objects);
    println!("> '{}M' call/s ({}%)", 
        for_spd.1.round(),
        fn_avg(for_spd.1, vec_spd.1)
    );

    let ecs_spd = ecs_bencher(run_id, objects);
    println!("> '{}M' call/s ({}%)", 
        ecs_spd.1.round(), 
        fn_avg(ecs_spd.1, vec_spd.1)
    ); 

    (vec_spd, for_spd, ecs_spd)
}

struct Bench(Vec<(usize, f64)>);

impl Bench {
    pub fn merge(mut self, other: Bench) -> Self {
        for v in other.0 {
            self.0.push(v);
        }
        self
    }

    pub fn avg(&self) -> f64 { self.0.iter().map(|x| x.1).sum::<f64>() / self.0.len() as f64 }

    pub fn min(&self) -> (usize, f64) { 
        let min = *self.0.iter().max_by(|a, b| cmp(&a.1, &b.1)).unwrap();
        //let pos = self.0.iter().position(|x| x.1 == min).unwrap();
        //self.0[pos]
        min
    }

    pub fn max(&self) -> (usize, f64) { 
        let max = *self.0.iter().min_by(|a, b| cmp(&a.1, &b.1)).unwrap();
        //let pos = self.0.iter().position(|x| x.1 == max).unwrap();
        //self.0[pos]
        max
    }
}

pub fn cmp(a:&f64, b:&f64) -> Ordering { 
    if a < b { Ordering::Greater } else { Ordering::Less }
}

// fn add_system(spawn: &Spawn, swarm: &mut Swarm<T>) {
//     swarm.get_mut(spawn).add_one();
// }


fn vec_bencher(id: &mut usize, amount: usize) -> (usize, f64) {
    *id += 1;
    print!("{}: Running Standard Vec bench for {} objects",id, amount);
    // get 'standard vector' thread speed
    let mut vec_test = vec![Minion::default(); amount];
    let now = std::time::SystemTime::now();
    for _j in 0..1_000 { 
        for k in 0..amount {
            vec_test[k].add_one();
        }
    }
    let elapsed_vec = now.elapsed();
    // base test results
    let time = (elapsed_vec.unwrap().as_nanos() as f64) * 0.001;
    let speed =  (vec_test[0].times_summoned * amount as u128) as f64 / time;
    // test / output
    assert_eq!(vec_test[0].times_summoned, 1_000);
    //println!("   - bench vec: speed was {}M calls/s", speed.round());
    // return result
    (id.clone(), speed)
}

fn for_bencher(id: &mut usize, amount: usize) -> (usize, f64) {
    *id += 1;
    print!("{}: Running Swarm foreach bench for {} objects", id, amount);
    // get swarm ecs system speed
    let mut swarm: swarm::Swarm<Minion> = Swarm::new(1_000_000);
    for _e in 0..amount { swarm.spawn(); }

    let now = std::time::SystemTime::now();
    for _j in 0..1_000 { 
        swarm.for_each(|obj| obj.times_summoned += 1);
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = (elapsed_res.unwrap().as_nanos() as f64) * 0.001;
    let swarm_speed = (swarm.get_mut(&0).times_summoned * amount as u128) as f64 / swarm_time;

    assert_eq!(swarm.get_mut(&0).times_summoned, 1_000);
    if amount > 1 {
        assert_eq!(swarm.get_mut(&1).times_summoned, 1_000);
    } else if amount > 3 {
        assert_eq!(swarm.get_mut(&(amount-2)).times_summoned, 1_000);
        assert_eq!(swarm.get_mut(&(amount-1)).times_summoned, 1_000);
    }

    (id.clone(), swarm_speed)
}

fn ecs_bencher(id: &mut usize, amount: usize) -> (usize, f64) {
    *id += 1;
    print!("{}: Running Swarm Ecs bench for {} objects", id, amount);
    // get swarm ecs system speed
    let mut swarm: Swarm<Minion> = Swarm::new(1_000_000);
    let mut add_system = AddSystem;
    AddSystem::add_requirement(1);

    for _e in 0..amount { 
        let spawn = swarm.spawn().unwrap();
        swarm.add_component(&spawn, 1); 
    }

    let now = std::time::SystemTime::now();
    for _j in 0..1_000 { 
        add_system.run(&mut swarm);
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = (elapsed_res.unwrap().as_nanos() as f64) * 0.001;
    let swarm_speed = (swarm.get_mut(&0).times_summoned * amount as u128) as f64 / swarm_time;
    
    assert_eq!(swarm.get_mut(&0).times_summoned, 1_000);
    if amount > 1 {
        assert_eq!(swarm.get_mut(&1).times_summoned, 1_000);
    } else if amount > 3 {
        assert_eq!(swarm.get_mut(&(amount-2)).times_summoned, 1_000);
        assert_eq!(swarm.get_mut(&(amount-1)).times_summoned, 1_000);
    }

    (id.clone(), swarm_speed)
}

