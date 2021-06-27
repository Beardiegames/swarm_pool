
#[allow(unused_imports)]
use std::cmp::Ordering;
use swarm::{ Swarm, Spawn };
use swarm::ecs::{ Component, System, SystemBuilder };

#[derive(Default, Copy, Clone)]
pub struct Summoning {
    times_summoned: u128,
}

impl Summoning {
    pub fn add_one(&mut self) {
        self.times_summoned += 1;
    }
}

#[derive(Default, Copy, Clone)]
pub struct Minion {
    times_summoned: u128,
    summon: Option<Summoning>,
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
pub struct AddSystem2;

impl System<Minion> for AddSystem2 {

    fn update(&mut self, spawn: &Spawn, swarm: &mut Swarm<Minion>) {
        swarm.get_mut(spawn).times_summoned += 1;
    }
}
pub struct AddSystem3;

impl System<Minion> for AddSystem3 {

    fn update(&mut self, spawn: &Spawn, swarm: &mut Swarm<Minion>) {
        swarm.get_mut(spawn).times_summoned += 1;
    }
}

fn main() {
    let mut run_id: usize = 0;

    let (vec_bn1, for_bn1, ecs_bn1, opt_bn1) = bench_with_objects(&mut run_id);
    let (vec_bn2, for_bn2, ecs_bn2, opt_bn2) = bench_with_objects(&mut run_id);
    let (vec_bn3, for_bn3, ecs_bn3, opt_bn3) = bench_with_objects(&mut run_id);
    let (vec_bn4, for_bn4, ecs_bn4, opt_bn4) = bench_with_objects(&mut run_id);
    let (vec_bn5, for_bn5, ecs_bn5, opt_bn5) = bench_with_objects(&mut run_id);

    let bvec = vec_bn1
        .merge(vec_bn2)
        .merge(vec_bn3)
        .merge(vec_bn4)
        .merge(vec_bn5);
    let bfor = for_bn1
        .merge(for_bn2)
        .merge(for_bn3)
        .merge(for_bn4)
        .merge(for_bn5);
    let becs = ecs_bn1
        .merge(ecs_bn2)
        .merge(ecs_bn3)
        .merge(ecs_bn4)
        .merge(ecs_bn5);
    let bopt = opt_bn1
        .merge(opt_bn2)
        .merge(opt_bn3)
        .merge(opt_bn4)
        .merge(opt_bn5);

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
    
    println!("Swarm option results:");
    println!(" - average of '{}M' calls/s", bopt.avg().round());
    println!(" - average speed was '{}%' of plain vector speed", 
        ((bopt.avg() / bvec.avg()) * 100_000.0).round() / 1_000.0);

    let vmin = bopt.min();
    println!(" - lowest of '{}M' calls/s -> becnh #{}", 
        vmin.1.round(), vmin.0);
    let vmax = bopt.max();
    println!(" - highest of '{}M' calls/s -> becnh #{}", 
        vmax.1.round(), vmax.0);

}

fn bench_with_objects(run_id: &mut usize) -> (Bench, Bench, Bench, Bench) {
    
    let (v_spd1, f_spd1, e_spd1, o_spd1) = bench_with(run_id, 1);
    let (v_spd2, f_spd2, e_spd2, o_spd2) = bench_with(run_id, 10);
    let (v_spd3, f_spd3, e_spd3, o_spd3) = bench_with(run_id, 100);
    let (v_spd4, f_spd4, e_spd4, o_spd4) = bench_with(run_id, 1_000);
    let (v_spd5, f_spd5, e_spd5, o_spd5) = bench_with(run_id, 10_000);
    let (v_spd6, f_spd6, e_spd6, o_spd6) = bench_with(run_id, 100_000);
    let (v_spd7, f_spd7, e_spd7, o_spd7) = bench_with(run_id, 1_000_000);

    println!("--");
    (   
        Bench (vec![v_spd1, v_spd2, v_spd3, v_spd4, v_spd5, v_spd6, v_spd7]),
        Bench (vec![f_spd1, f_spd2, f_spd3, f_spd4, f_spd5, f_spd6, f_spd7]),
        Bench (vec![e_spd1, e_spd2, e_spd3, e_spd4, e_spd5, e_spd6, e_spd7]),
        Bench (vec![o_spd1, o_spd2, o_spd3, o_spd4, o_spd5, o_spd6, o_spd7]),
    )
}

fn bench_with(run_id: &mut usize, objects: usize) -> ((usize, f64), (usize, f64), (usize, f64), (usize, f64)) {
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

    let opt_spd = option_bencher(run_id, objects);
    println!("> '{}M' call/s ({}%)", 
        opt_spd.1.round(), 
        fn_avg(opt_spd.1, vec_spd.1)
    ); 

    (vec_spd, for_spd, ecs_spd, opt_spd)
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
    for _j in 0..1_200 { 
        for k in 0..amount {
            vec_test[k].add_one();
        }
    }
    let elapsed_vec = now.elapsed();
    // base test results
    let time = (elapsed_vec.unwrap().as_nanos() as f64) * 0.001;
    let speed =  (vec_test[0].times_summoned * amount as u128) as f64 / time;
    // test / output
    assert_eq!(vec_test[0].times_summoned, 1_200);
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
    for _j in 0..1_200 { 
        swarm.for_each(|obj| obj.times_summoned += 1);
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = (elapsed_res.unwrap().as_nanos() as f64) * 0.001;
    let swarm_speed = (swarm.get_mut(&0).times_summoned * amount as u128) as f64 / swarm_time;

    assert_eq!(swarm.get_mut(&0).times_summoned, 1_200);
    if amount > 1 {
        assert_eq!(swarm.get_mut(&1).times_summoned, 1_200);
    } else if amount > 3 {
        assert_eq!(swarm.get_mut(&(amount-2)).times_summoned, 1_200);
        assert_eq!(swarm.get_mut(&(amount-1)).times_summoned, 1_200);
    }

    (id.clone(), swarm_speed)
}

fn ecs_bencher(id: &mut usize, amount: usize) -> (usize, f64) {
    *id += 1;
    print!("{}: Running Swarm Ecs bench for {} objects", id, amount);
    // get swarm ecs system speed
    let mut swarm: Swarm<Minion> = Swarm::new(1_000_000);
    let mut add_system = SystemBuilder::new(AddSystem)
        .requires_component(Component::new(0))
        .build();
    let mut add_system2 = SystemBuilder::new(AddSystem2)
        .requires_component(Component::new(1))
        .build();
    let mut add_system3 = SystemBuilder::new(AddSystem3)
        .requires_component(Component::new(2))
        .build();

    for _e in 0..amount { 
        let spawn = swarm.spawn().unwrap();
        swarm.add_component(&spawn, Component::new(0)); 
        swarm.add_component(&spawn, Component::new(1)); 
        swarm.add_component(&spawn, Component::new(2)); 
    }

    let now = std::time::SystemTime::now();
    for _j in 0..400 { 
        add_system.run(&mut swarm);
        add_system2.run(&mut swarm);
        add_system3.run(&mut swarm);
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = (elapsed_res.unwrap().as_nanos() as f64) * 0.001;
    let swarm_speed = (swarm.get_mut(&0).times_summoned * amount as u128) as f64 / swarm_time;
    
    assert_eq!(swarm.get_mut(&0).times_summoned, 1_200);
    if amount > 1 {
        assert_eq!(swarm.get_mut(&1).times_summoned, 1_200);
    } else if amount > 3 {
        assert_eq!(swarm.get_mut(&(amount-2)).times_summoned, 1_200);
        assert_eq!(swarm.get_mut(&(amount-1)).times_summoned, 1_200);
    }

    (id.clone(), swarm_speed)
}

fn option_bencher(id: &mut usize, amount: usize) -> (usize, f64) {
    *id += 1;
    print!("{}: Running Swarm Option bench for {} objects", id, amount);
    // get swarm ecs system speed
    let mut swarm: Swarm<Minion> = Swarm::new(1_000_000);
    for _e in 0..amount { 
        let spawn = swarm.spawn().unwrap();
        swarm.get_mut(&spawn).summon = Some(Summoning::default());
    }

    let now = std::time::SystemTime::now();
    for _j in 0..1_200 { 
        swarm.for_each(|obj| {
            if let Some(summon) = &mut obj.summon {
                summon.times_summoned += 1;
            }
        });
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = (elapsed_res.unwrap().as_nanos() as f64) * 0.001;
    let swarm_speed = (swarm.get_mut(&0).summon.unwrap().times_summoned * amount as u128) as f64 / swarm_time;
    
    assert_eq!(swarm.get_mut(&0).summon.unwrap().times_summoned, 1_200);
    if amount > 1 {
        assert_eq!(swarm.get_mut(&1).summon.unwrap().times_summoned, 1_200);
    } else if amount > 3 {
        assert_eq!(swarm.get_mut(&(amount-2)).summon.unwrap().times_summoned, 1_200);
        assert_eq!(swarm.get_mut(&(amount-1)).summon.unwrap().times_summoned, 1_200);
    }

    (id.clone(), swarm_speed)
}

