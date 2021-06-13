
#[allow(unused_imports)]
use std::cmp::Ordering;

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
    let (vec_bn1, for_bn1, ecs_bn1) = bench_with_objects();
    let (vec_bn2, for_bn2, ecs_bn2) = bench_with_objects();
    let (vec_bn3, for_bn3, ecs_bn3) = bench_with_objects();

    let bvec = vec_bn1.merge(vec_bn2).merge(vec_bn3);
    let bfor = for_bn1.merge(for_bn2).merge(for_bn3);
    let becs = ecs_bn1.merge(ecs_bn2).merge(ecs_bn3);

    println!("# RESULTS TOTAL:");

    println!("Plain vec results:");
    println!(" - average of '{}M' calls/s", bvec.avg().round());

    let vmin = bvec.min();
    println!(" - lowest of '{}M' calls/s -> {} objects", 
        vmin.0.round(), vmin.1);
    let vmax = bvec.max();
    println!(" - highest of '{}M' calls/s -> {} objects", 
        vmax.0.round(), vmax.1);
    
    println!("Swarm foreach results:");
    println!(" - average of '{}M' calls/s", bfor.avg().round());
    println!(" - av. speed was '{}%' of plain vector speed", 
        ((bfor.avg() / bvec.avg()) * 100_000.0).round() / 1_000.0);

    let vmin = bfor.min();
    println!(" - lowest of '{}M' calls/s -> {} objects", 
        vmin.0.round(), vmin.1);
    let vmax = bfor.max();
    println!(" - highest of '{}M' calls/s -> {} objects", 
        vmax.0.round(), vmax.1);
    

    println!("Swarm ecs results:");
    println!(" - average of '{}M' calls/s", becs.avg().round());
    println!(" - average speed was '{}%' of plain vector speed", 
        ((becs.avg() / bvec.avg()) * 100_000.0).round() / 1_000.0);

    let vmin = becs.min();
    println!(" - lowest of '{}M' calls/s -> {} objects", 
        vmin.0.round(), vmin.1);
    let vmax = becs.max();
    println!(" - highest of '{}M' calls/s -> {} objects", 
        vmax.0.round(), vmax.1);
    
}

fn bench_with_objects() -> (Bench, Bench, Bench) {
    
    let (v_spd1, f_spd1, e_spd1) = bench_with(1);
    let (v_spd2, f_spd2, e_spd2) = bench_with(10);
    let (v_spd2, f_spd2, e_spd2) = bench_with(100);
    let (v_spd3, f_spd3, e_spd3) = bench_with(1_000);
    let (v_spd4, f_spd4, e_spd4) = bench_with(10_000);
    let (v_spd5, f_spd5, e_spd5) = bench_with(100_000);
    let (v_spd6, f_spd6, e_spd6) = bench_with(1_000_000);

    println!("--");
    (   
        Bench (vec![v_spd1, v_spd2, v_spd3, v_spd4, v_spd5, v_spd6]),
        Bench (vec![f_spd1, f_spd2, f_spd3, f_spd4, f_spd5, f_spd6]),
        Bench (vec![e_spd1, e_spd2, e_spd3, e_spd4, e_spd5, e_spd6]),
    )
}

fn bench_with(objects: usize) -> (f64, f64, f64) {
    let fn_spd = |x: f64, vec: f64| (100.0 * x / vec).round();

    std::thread::sleep(std::time::Duration::from_millis(500));
    //println!("START BENCH over {} objects using {} test cycles", objects, cycles);
    
    let vec_spd = vec_bencher(objects);
    println!("> '{}M' call/s", vec_spd.round());

    let for_spd = for_bencher(objects);
    println!("> '{}M' call/s ({}%)", 
        for_spd.round(),
        fn_spd(for_spd, vec_spd)
    );

    let ecs_spd = ecs_bencher(objects);
    println!("> '{}M' call/s ({}%)", 
        ecs_spd.round(), 
        fn_spd(ecs_spd, vec_spd)
    ); 

    (vec_spd, for_spd, ecs_spd)
}

struct Bench(Vec<f64>);

impl Bench {
    pub fn merge(mut self, other: Bench) -> Self {
        for v in other.0 {
            self.0.push(v);
        }
        self
    }

    pub fn avg(&self) -> f64 { self.0.iter().sum::<f64>() / 7.0 }

    pub fn min(&self) -> (f64, usize) { 
        let min = *self.0.iter().max_by(|a, b| cmp(a, b)).unwrap();
        let pos = self.0.iter().position(|x| *x == min).unwrap();
        (min, pos)
    }

    pub fn max(&self) -> (f64, usize) { 
        let max = *self.0.iter().min_by(|a, b| cmp(a, b)).unwrap();
        let pos = self.0.iter().position(|x| *x == max).unwrap();
        (max, pos)
    }
}

pub fn cmp(a:&f64, b:&f64) -> Ordering { 
    if a < b { Ordering::Greater } else { Ordering::Less }
}

fn add_system(x: &mut Minion) {
    x.add_one();
}


fn vec_bencher(amount: usize) -> f64 {
    print!("Running Standard Vec bench for {} objects", amount);
    // get 'standard vector' thread speed
    let mut vec_test = vec![Minion::default(); amount];
    let now = std::time::SystemTime::now();
    for _i in 0..10 {
        for _j in 0..1_000 { 
            for k in 0..amount {
                vec_test[k].add_one();
            }
        }
        
        print!(".");
    }
    let elapsed_vec = now.elapsed();
    // base test results
    let time = (elapsed_vec.unwrap().as_nanos() as f64) * 0.001;
    let speed =  (vec_test[0].times_summoned * amount as u128) as f64 / time;
    // test / output
    assert_eq!(vec_test[0].times_summoned, 10_000);
    //println!("   - bench vec: speed was {}M calls/s", speed.round());
    // return result
    speed
}

fn for_bencher(amount: usize) -> f64 {
    print!("Running Swarm foreach bench for {} objects", amount);
    // get swarm ecs system speed
    let mut swarm: swarm::Swarm<Minion> = swarm::Swarm::new(1_000_000);
    for _e in 0..amount { swarm.spawn(); }

    let now = std::time::SystemTime::now();
    for _i in 0..10 {
        for _j in 0..1_000 { 
            swarm.for_each(add_system);
        }
        print!(".");
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = (elapsed_res.unwrap().as_nanos() as f64) * 0.001;
    let swarm_speed = (swarm.get_mut(&0).times_summoned * amount as u128) as f64 / swarm_time;

    assert_eq!(swarm.get_mut(&0).times_summoned, 10_000);
    if amount > 1 {
        assert_eq!(swarm.get_mut(&1).times_summoned, 10_000);
    } else if amount > 3 {
        assert_eq!(swarm.get_mut(&(amount-2)).times_summoned, 10_000);
        assert_eq!(swarm.get_mut(&(amount-1)).times_summoned, 10_000);
    }

    swarm_speed
}

fn ecs_bencher(amount: usize) -> f64 {

    print!("Running Swarm Ecs bench for {} objects", amount);
    // get swarm ecs system speed
    let mut swarm: swarm::Swarm<Minion> = swarm::Swarm::new(1_000_000);
    let mut add_system: swarm::ecs::System<Minion> = swarm::ecs::System::new(&[0], add_system);
    for _e in 0..amount { swarm.spawn(); }

    let now = std::time::SystemTime::now();
    for _i in 0..10 {
        for _j in 0..1_000 { 
            add_system.run(&mut swarm);
        }
        print!(".");
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = (elapsed_res.unwrap().as_nanos() as f64) * 0.001;
    let swarm_speed = (swarm.get_mut(&0).times_summoned * amount as u128) as f64 / swarm_time;
    
    assert_eq!(swarm.get_mut(&0).times_summoned, 10_000);
    if amount > 1 {
        assert_eq!(swarm.get_mut(&1).times_summoned, 10_000);
    } else if amount > 3 {
        assert_eq!(swarm.get_mut(&(amount-2)).times_summoned, 10_000);
        assert_eq!(swarm.get_mut(&(amount-1)).times_summoned, 10_000);
    }

    swarm_speed
}

