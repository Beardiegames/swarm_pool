
#[allow(unused_imports)]
use std::cmp::Ordering;
use std::io::Write;
use swarm::*;

#[derive(Default, Copy, Clone, Debug)]
pub struct Summoning {
    times_summoned: u128,
}

impl Summoning {
    pub fn add_one(&mut self) {
        self.times_summoned += 1;
    }

}

pub struct SwarmData;

#[derive(Default, Debug, Copy, Clone)]
pub struct Minion {
    times_summoned: u128,
    summon: Option<Summoning>,
}

impl Minion {
    pub fn add_one(&mut self) {
        self.times_summoned += 1;
    }
}


const NUM_SAMPLES: usize = 5_000_000_000;


fn main() {
    let mut run_id: usize = 0;
    let (vec_bn1, for_h_bn1, upd_h_bn1, upc_h_bn1) = bench_with_objects(&mut run_id);

    println!("# RESULTS TOTAL:");

    println!("* Plain vec results:");
        println!("  - average of '{}M' calls/s", (vec_bn1.avg() / 1_000_000.0).round());
        let vmin = vec_bn1.min();
        println!("  - lowest of '{}M' calls/s (bench #{})", 
            (vmin.1 / 1_000_000.0).round(), vmin.0);
        let vmax = vec_bn1.max();
        println!("  - highest of '{}M' calls/s (becnh #{})", 
            (vmax.1 / 1_000_000.0).round(), vmax.0);
    
    println!("* swarm.for_each() results:");
        println!("  - average of '{}M' calls/s", (for_h_bn1.avg() / 1_000_000.0).round());
        println!("  - av. speed was '{}%' of plain vector speed", 
            ((for_h_bn1.avg() / vec_bn1.avg()) * 100_000.0).round() / 1_000.0);
        let vmin = for_h_bn1.min();
        println!("  - lowest of '{}M' calls/s (bench #{})", 
            (vmin.1 / 1_000_000.0).round(), vmin.0);
        let vmax = for_h_bn1.max();
        println!("  - highest of '{}M' calls/s (bench #{})", 
            (vmax.1 / 1_000_000.0).round(), vmax.0);
        
    println!("* swarm.for_all() results:");
        println!("  - average of '{}M' calls/s", (upd_h_bn1.avg() / 1_000_000.0).round());
        println!("  - average speed was '{}%' of plain vector speed", 
            ((upd_h_bn1.avg() / vec_bn1.avg()) * 100_000.0).round() / 1_000.0);
        let vmin = upd_h_bn1.min();
        println!("  - lowest of '{}M' calls/s (becnh #{})", 
            (vmin.1 / 1_000_000.0).round(), vmin.0);
        let vmax = upd_h_bn1.max();
        println!("  - highest of '{}M' calls/s (becnh #{})", 
            (vmax.1 / 1_000_000.0).round(), vmax.0);

    println!("* swarm.update() results:");
        println!("  - average of '{}M' calls/s", (upc_h_bn1.avg() / 1_000_000.0).round());
        println!("  - average speed was '{}%' of plain vector speed", 
            ((upc_h_bn1.avg() / vec_bn1.avg()) * 100_000.0).round() / 1_000.0);

        let vmin = upc_h_bn1.min();
        println!("  - lowest of '{}M' calls/s (becnh #{})", 
            (vmin.1 / 1_000_000.0).round(), vmin.0);
        let vmax = upc_h_bn1.max();
        println!("  - highest of '{}M' calls/s (becnh #{})", 
            (vmax.1 / 1_000_000.0).round(), vmax.0);
}

fn bench_with_objects(run_id: &mut usize) -> (Bench, Bench, Bench, Bench) {
    
    let (v_spd1, fh_spd1, uh_spd1, ch_spd1) = bench_with(run_id, 1);
    let (v_spd2, fh_spd2, uh_spd2, ch_spd2) = bench_with(run_id, 10);
    let (v_spd3, fh_spd3, uh_spd3, ch_spd3) = bench_with(run_id, 100);
    let (v_spd4, fh_spd4, uh_spd4, ch_spd4) = bench_with(run_id, 1_000);
  
    println!("--");
    (   
        Bench (vec![v_spd1, v_spd2, v_spd3, v_spd4]),
        Bench (vec![fh_spd1, fh_spd2, fh_spd3, fh_spd4]),
        Bench (vec![uh_spd1, uh_spd2, uh_spd3, uh_spd4]),
        Bench (vec![ch_spd1, ch_spd2, ch_spd3, ch_spd4]),
    )
}

type Speed = (usize, f64);

fn bench_with(run_id: &mut usize, objects: usize) -> (Speed, Speed, Speed, Speed) {
    let fn_avg = |x: f64, vec: f64| (100.0 * x / vec).round();

    std::thread::sleep(std::time::Duration::from_millis(500));
    
    let vec_spd = vec_heap_bencher(run_id, objects);
    {
        let m_calls = (vec_spd.1 / 1_000_000.0).round();
        println!("{}M calls/s @ {}M upd/s", m_calls, m_calls / objects as f64);
    }

    let for_h_spd = for_heap_bencher(run_id, objects);
    {
        let m_calls = (for_h_spd.1 / 1_000_000.0).round();
        let avg = fn_avg(for_h_spd.1, vec_spd.1);
        println!("{}M calls/s({}%) @ {}M upd/s", m_calls, avg, m_calls / objects as f64);
    }

    let upd_h_spd = forall_heap_bencher(run_id, objects);
    {
        let m_calls = (upd_h_spd.1 / 1_000_000.0).round();
        let avg = fn_avg(upd_h_spd.1, vec_spd.1);
        println!("{}M calls/s({}%) @ {}M upd/s", m_calls, avg, m_calls / objects as f64);
    }
    let uct_h_spd = update_heap_bencher(run_id, objects);
    {
        let m_calls = (uct_h_spd.1 / 1_000_000.0).round();
        let avg = fn_avg(uct_h_spd.1, vec_spd.1);
        println!("{}M calls/s({}%) @ {}M upd/s", m_calls, avg, m_calls / objects as f64);
    } 

    (vec_spd, for_h_spd, upd_h_spd, uct_h_spd)
}

struct Bench(Vec<Speed>);

impl Bench {
    // pub fn merge(mut self, other: Bench) -> Self {
    //     for v in other.0 {
    //         self.0.push(v);
    //     }
    //     self
    // }

    pub fn avg(&self) -> f64 { self.0.iter().map(|x| x.1).sum::<f64>() / self.0.len() as f64 }

    pub fn min(&self) -> Speed { 
        let min = *self.0.iter().max_by(|a, b| cmp(&a.1, &b.1)).unwrap();
        min
    }

    pub fn max(&self) -> Speed { 
        let max = *self.0.iter().min_by(|a, b| cmp(&a.1, &b.1)).unwrap();
        max
    }
}

pub fn cmp(a:&f64, b:&f64) -> Ordering { 
    if a < b { Ordering::Greater } else { Ordering::Less }
}

fn vec_heap_bencher(id: &mut usize, amount: usize) -> Speed {
    *id += 1;
    print!("{}: Standard Vec bench for {} object(s).. ",id, amount);
    std::io::stdout().flush();

    // get 'standard vector' thread speed
    let mut vec_test = vec![Minion::default(); amount];

    let now = std::time::SystemTime::now();
    for _j in 0..NUM_SAMPLES/amount { 
        for k in 0..amount {
            vec_test[k].times_summoned += 1;
        }
    }
    let elapsed_vec = now.elapsed();

    // base test results
    let time = elapsed_vec.unwrap().as_secs_f64();
    let speed = (vec_test[0].times_summoned * amount as u128) as f64 / time;

    // return result
    (id.clone(), speed)
}

fn for_heap_bencher(id: &mut usize, amount: usize) -> Speed {
    *id += 1;
    print!("{}: Swarm.for_each() bench with {} object(s).. ", id, amount);
    std::io::stdout().flush();

    // get swarm ecs system speed
    let mut swarm = Swarm::<Minion, _>::new(amount, ());
    let s_first = swarm.spawn().unwrap();
    for _e in 1..amount { swarm.spawn(); }

    // run bench loop
    let now = std::time::SystemTime::now();
    for _j in 0..NUM_SAMPLES/amount { 
        swarm.for_each(|obj| {
            obj.times_summoned += 1;
        });
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = elapsed_res.unwrap().as_secs_f64();
    let swarm_speed = (swarm.get_mut(&s_first).times_summoned * amount as u128) as f64 / swarm_time;

    (id.clone(), swarm_speed)
}

fn forall_heap_bencher(id: &mut usize, amount: usize) -> Speed {
    *id += 1;
    print!("{}: Swarm.for_all() bench with {} object(s).. ", id, amount);
    std::io::stdout().flush();
    // get swarm ecs system speed
    let mut swarm = Swarm::<Minion, _>::new(amount, ());
    let s_first = swarm.spawn().unwrap();
    for _e in 1..amount { 
        let spawn = swarm.spawn().unwrap();
        swarm.get_mut(&spawn).summon = Some(Summoning::default());
    }

    // run bench loop
    let now = std::time::SystemTime::now();
    for _j in 0..NUM_SAMPLES/amount { 
        swarm.for_all(|index, list, _props| {
            list[*index].times_summoned += 1;
        });
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = elapsed_res.unwrap().as_secs_f64();
    let swarm_speed = (swarm.get_mut(&s_first).times_summoned * amount as u128) as f64 / swarm_time;

    (id.clone(), swarm_speed)
}

fn update_heap_bencher(id: &mut usize, amount: usize) -> Speed {
    *id += 1;
    print!("{}: Swarm.update() bench with {} object(s).. ", id, amount);
    std::io::stdout().flush();

    // get swarm ecs system speed
    let mut swarm = Swarm::<Minion, _>::new(amount, ());
    let s_first = swarm.spawn().unwrap();
    for _e in 1..amount { 
        let spawn = swarm.spawn().unwrap();
        swarm.get_mut(&spawn).summon = Some(Summoning::default());
    }

    // run bench loop
    let now = std::time::SystemTime::now();
    for _j in 0..NUM_SAMPLES/amount { 
        swarm.update(|spawn, swarm| {
            swarm.raw_mut(spawn).times_summoned += 1;
        });
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = elapsed_res.unwrap().as_secs_f64();
    let swarm_speed = (swarm.get_mut(&s_first).times_summoned * amount as u128) as f64 / swarm_time;

    (id.clone(), swarm_speed)
}



// fn for_stack_bencher(id: &mut usize, amount: usize) -> Speed {
//     *id += 1;
//     println!("{}: Running Stack Swarm foreach bench for {} objects", id, amount);
//     // get swarm ecs system speed
//     let mut swarm = StackSwarm::<Minion, SwarmData>::new();
//     let spawns = match amount { a if a <= 1_000 => a, _ => 1_000, };

//     for _e in 0..spawns { swarm.spawn(); }

//     let now = std::time::SystemTime::now();
//     for _j in 0..NUM_SAMPLES/amount { 
//         swarm.for_each(|obj| {
//             obj.times_summoned += 1;
//         });
//     }
//     let elapsed_res = now.elapsed();

//     // swarm test results
//     let swarm_time = elapsed_res.unwrap().as_secs_f64();
//     let swarm_speed = (swarm.get_mut(&0).times_summoned * amount as u128) as f64 / swarm_time;

//     (id.clone(), swarm_speed)
// }

// fn update_stack_bencher(id: &mut usize, amount: usize) -> Speed {
//     *id += 1;
//     println!("{}: Running Stack Swarm Update bench for {} objects", id, amount);
//     // get swarm ecs system speed
//     let mut swarm = StackSwarm::<Minion, SwarmData>::new();
//     let spawns = match amount { a if a <= a => a, _ => 1_000, };

//     for _e in 0..spawns { 
//         let spawn = swarm.spawn().unwrap();
//         swarm.get_mut(&spawn).summon = Some(Summoning::default());
//     }

//     let now = std::time::SystemTime::now();
//     for _i in 0..NUM_SAMPLES/amount {
//         swarm.update(|ptr, swarm| {
//             swarm[*ptr].times_summoned += 1;
//         });
//     }
//     let elapsed_res = now.elapsed();

//     // swarm test results
//     let swarm_time = elapsed_res.unwrap().as_secs_f64();
//     let swarm_speed = (swarm.get_mut(&0).times_summoned * spawns as u128) as f64 / swarm_time;

//     (id.clone(), swarm_speed)
// }

// fn update_ctl_stack_bencher(id: &mut usize, amount: usize) -> Speed {
//     *id += 1;
//     println!("{}: Running Stack Swarm UpdateCTL bench for {} objects", id, amount);
//     // get swarm ecs system speed
//     let mut swarm = StackSwarm::<Minion, SwarmData>::new();
//     let spawns = match amount { a if a <= a => a, _ => 1_000, };

//     for _e in 0..spawns { 
//         let spawn = swarm.spawn().unwrap();
//         swarm.get_mut(&spawn).summon = Some(Summoning::default());
//     }

//     let now = std::time::SystemTime::now();
//     for _i in 0..NUM_SAMPLES/amount {
//         swarm.update_ctl(|spawn, swarm| {
//             swarm.get_mut(spawn).times_summoned += 1;
//         });
//     }
//     let elapsed_res = now.elapsed();

//     // swarm test results
//     let swarm_time = elapsed_res.unwrap().as_secs_f64();
//     let swarm_speed = (swarm.get_mut(&0).times_summoned * spawns as u128) as f64 / swarm_time;

//     (id.clone(), swarm_speed)
// }

