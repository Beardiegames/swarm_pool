
#[allow(unused_imports)]
use std::cmp::Ordering;
use std::io::Write;

extern crate swarm_pool;
use swarm_pool::Swarm;
//use swarm_pool::tools::sized_pool::SizedPool1024;
//use swarm_pool::tools::sized_pool;

// test mockup objects

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Summon {
    calls: u128,
}

impl Summon {
    pub fn add_one(&mut self) {
        self.calls += 1;
    }
}

pub struct SwarmData;

#[derive(Default, Debug, Copy, Clone)]
pub struct Minion {
    calls: u128,
    summon: Option<Summon>,
}

impl Minion {
    pub fn add_one(&mut self) {
        self.calls += 1;
    }
}

// bench test settings

const NUM_SAMPLES: u128 = 2_000_000_000;

// bench implementation

fn main() {
    let mut run_id: usize = 0;
    let (vec_bn1, for_h_bn1, upd_h_bn1, upc_h_bn1) = bench_with_objects(&mut run_id);

    println!("# RESULTS TOTAL:");

    println!("* Plain vec results:");
    {
        println!("  - average of '{}M' calls/s", (vec_bn1.avg() / 1_000_000.0).round());
        let vmin = vec_bn1.min();
        println!("  - lowest of '{}M' calls/s (bench #{})", 
            (vmin.1 / 1_000_000.0).round(), vmin.0);
        let vmax = vec_bn1.max();
        println!("  - highest of '{}M' calls/s (becnh #{})", 
            (vmax.1 / 1_000_000.0).round(), vmax.0);
    }
    print_result(&vec_bn1, &for_h_bn1, "swarm.for_each()"); 
    print_result(&vec_bn1, &upd_h_bn1, "swarm.for_all()"); 
    print_result(&vec_bn1, &upc_h_bn1, "swarm.update()"); 
    //print_result(&vec_bn1, &pool_bench, "pool::for_each()"); 

}

fn print_result(baseline: &Bench, test_bench: &Bench, descript: &str) {
    println!("* {} results:", descript); 

    println!("  - average of '{}M' calls/s", (test_bench.avg() / 1_000_000.0).round());
    println!("  - average speed was '{}%' of plain vector speed", 
        ((test_bench.avg() / baseline.avg()) * 100_000.0).round() / 1_000.0);

    let vmin = test_bench.min();
    println!("  - lowest of '{}M' calls/s (becnh #{})", 
        (vmin.1 / 1_000_000.0).round(), vmin.0);
    let vmax = test_bench.max();
    println!("  - highest of '{}M' calls/s (becnh #{})", 
        (vmax.1 / 1_000_000.0).round(), vmax.0);
}

fn bench_with_objects(run_id: &mut usize) -> (Bench, Bench, Bench, Bench) { //}, Bench) {
    
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
        //Bench (vec![p_spd1, p_spd2, p_spd3, p_spd4]),
    )
}

type Speed = (usize, f64);

fn bench_with(run_id: &mut usize, objects: u128) -> (Speed, Speed, Speed, Speed) {
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
    // let pool_spd = sized_pool_bencher(run_id, objects);
    // {
    //     let m_calls = (pool_spd.1 / 1_000_000.0).round();
    //     let avg = fn_avg(pool_spd.1, vec_spd.1);
    //     println!("{}M calls/s({}%) @ {}M upd/s", m_calls, avg, m_calls / objects as f64);
    // }
    

    (vec_spd, for_h_spd, upd_h_spd, uct_h_spd) //, pool_spd)
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

fn vec_heap_bencher(id: &mut usize, amount: u128) -> Speed {
    *id += 1;
    print!("{}: Standard Vec bench for {} object(s).. ",id, amount);
    #[allow(unused_must_use)] { std::io::stdout().flush(); }

    // get 'standard vector' thread speed
    let mut vec_test = vec![Minion::default(); amount as usize];

    let now = std::time::SystemTime::now();
    for _j in 0..NUM_SAMPLES/amount { 
        for k in 0..amount as usize {
            vec_test[k].calls += 1;
        }
    }
    let elapsed_vec = now.elapsed();

    // base test results
    let time = elapsed_vec.unwrap().as_secs_f64();
    let speed = (vec_test[0].calls * amount as u128) as f64 / time;
    assert_eq!(vec_test[0].calls, NUM_SAMPLES / amount as u128);

    // return result
    (*id, speed)
}

fn for_heap_bencher(id: &mut usize, amount: u128) -> Speed {
    *id += 1;
    print!("{}: Swarm.for_each() bench with {} object(s).. ", id, amount);
    #[allow(unused_must_use)] { std::io::stdout().flush(); }

    // get swarm ecs system speed
    let mut swarm = Swarm::<Minion, _>::new(amount as usize, ());
    let s_first = swarm.spawn().unwrap();
    for _e in 1..amount { swarm.spawn(); }

    // run bench loop
    let now = std::time::SystemTime::now();
    for _j in 0..NUM_SAMPLES/amount { 
        swarm.for_each(|obj| {
            obj.calls += 1;
        });
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = elapsed_res.unwrap().as_secs_f64();
    let swarm_speed = (swarm.fetch(&s_first).calls * amount as u128) as f64 / swarm_time;
    assert_eq!(swarm.fetch(&s_first).calls, NUM_SAMPLES / amount);

    (*id, swarm_speed)
}

fn forall_heap_bencher(id: &mut usize, amount: u128) -> Speed {
    *id += 1;
    print!("{}: Swarm.for_all() bench with {} object(s).. ", id, amount);
    #[allow(unused_must_use)] { std::io::stdout().flush(); }
    
    // get swarm ecs system speed
    let mut swarm = Swarm::<Minion, _>::new(amount as usize, ());
    let s_first = swarm.spawn().unwrap();

    for _e in 1..amount { 
        let spawn = swarm.spawn().unwrap();
        swarm.fetch(&spawn).summon = Some(Summon::default());
    }

    // run bench loop
    let now = std::time::SystemTime::now();
    for _j in 0..NUM_SAMPLES/amount { 
        swarm.for_all(|index, list, _props| {
            list[*index].calls += 1;
        });
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = elapsed_res.unwrap().as_secs_f64();
    let swarm_speed = (swarm.fetch(&s_first).calls * amount as u128) as f64 / swarm_time;
    assert_eq!(swarm.fetch(&s_first).calls, NUM_SAMPLES / amount);

    (*id, swarm_speed)
}

fn update_heap_bencher(id: &mut usize, amount: u128) -> Speed {
    *id += 1;
    print!("{}: Swarm.update() bench with {} object(s).. ", id, amount);
    #[allow(unused_must_use)] { std::io::stdout().flush(); }

    // get swarm ecs system speed
    let mut swarm = Swarm::<Minion, _>::new(amount as usize, ());
    let s_first = swarm.spawn().unwrap();

    for _e in 1..amount { 
        let spawn = swarm.spawn().unwrap();
        swarm.fetch(&spawn).summon = Some(Summon::default());
    }

    // run bench loop
    let now = std::time::SystemTime::now();
    for _j in 0..NUM_SAMPLES/amount { 
        swarm.update(|ctx| {
            ctx.target().calls += 1;
        });
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = elapsed_res.unwrap().as_secs_f64();
    let swarm_speed = (swarm.fetch(&s_first).calls * amount as u128) as f64 / swarm_time;
    assert_eq!(swarm.fetch(&s_first).calls, NUM_SAMPLES / amount);

    (*id, swarm_speed)
}

// fn sized_pool_bencher(id: &mut usize, amount: u128) -> Speed {
//     *id += 1;
//     print!("{}: pool::for_each() bench with {} object(s).. ", id, amount);
//     #[allow(unused_must_use)] { std::io::stdout().flush(); }

//     // get swarm ecs system speed
//     let mut pool: SizedPool1024<Summon> = SizedPool1024::new();

//     for _e in 0..amount { 
//        sized_pool::push(&mut pool, Summon::default());
//     }

//     // run bench loop
//     let now = std::time::SystemTime::now();
//     for _j in 0..NUM_SAMPLES/amount { 
//         sized_pool::for_each(&mut pool, |obj| obj.calls += 1);
//     }
//     let elapsed_res = now.elapsed();

//     // swarm test results
//     let swarm_time = elapsed_res.unwrap().as_secs_f64();
//     let num_calls = sized_pool::get_ref(&pool, 0).unwrap().calls;
//     let swarm_speed = (num_calls * amount as u128) as f64 / swarm_time;
//     assert_eq!(num_calls, NUM_SAMPLES / amount);

//     (*id, swarm_speed)
// }