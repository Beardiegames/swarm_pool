
#[allow(unused_imports)]
use std::cmp::Ordering;
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

    let (vec_bn1, for_h_bn1, upd_h_bn1, upc_h_bn1, for_s_bn1, upd_s_bn1, upc_s_bn1) = bench_with_objects(&mut run_id);
    // let (vec_bn2, for_h_bn2, upd_h_bn2, for_s_bn2, upd_s_bn2) = bench_with_objects(&mut run_id);
    // let (vec_bn3, for_h_bn3, upd_h_bn3, for_s_bn3, upd_s_bn3) = bench_with_objects(&mut run_id);
    // let (vec_bn4, for_h_bn4, upd_h_bn4, for_s_bn4, upd_s_bn4) = bench_with_objects(&mut run_id);
    // let (vec_bn5, for_h_bn5, upd_h_bn5, for_s_bn5, upd_s_bn5) = bench_with_objects(&mut run_id);

    let bvec = vec_bn1;
        //.merge(vec_bn2)
        //.merge(vec_bn3)
        //.merge(vec_bn4)
        //.merge(vec_bn5);
    let bforh = for_h_bn1;
        //.merge(for_h_bn2)
        //.merge(for_h_bn3)
        //.merge(for_h_bn4)
        //.merge(for_h_bn5);
    let bupdh = upd_h_bn1;
        //.merge(upd_h_bn2)
        //.merge(upd_h_bn3)
        //.merge(upd_h_bn4)
        //.merge(upd_h_bn5);
    let bupch = upc_h_bn1;

    let bfors = for_s_bn1;
        //.merge(for_s_bn2)
        //.merge(for_s_bn3)
        //.merge(for_s_bn4)
        //.merge(for_s_bn5);
    let bupds = upd_s_bn1;
        //.merge(upd_s_bn2)
        //.merge(upd_s_bn3)
        //.merge(upd_s_bn4)
        //.merge(upd_s_bn5);
    let bupcs = upc_s_bn1;


    println!("# RESULTS TOTAL:");

    println!("Plain vec results:");
        println!(" - average of '{}M' calls/s", (bvec.avg() / 1_000_000.0).round());
        let vmin = bvec.min();
        println!(" - lowest of '{}M' calls/s -> bench #{}", 
            (vmin.1 / 1_000_000.0).round(), vmin.0);
        let vmax = bvec.max();
        println!(" - highest of '{}M' calls/s -> becnh #{}", 
            (vmax.1 / 1_000_000.0).round(), vmax.0);
    
    println!("Swarm 'heap' foreach results:");
        println!(" - average of '{}M' calls/s", (bforh.avg() / 1_000_000.0).round());
        println!(" - av. speed was '{}%' of plain vector speed", 
            ((bforh.avg() / bvec.avg()) * 100_000.0).round() / 1_000.0);
        let vmin = bforh.min();
        println!(" - lowest of '{}M' calls/s -> bench #{}", 
            (vmin.1 / 1_000_000.0).round(), vmin.0);
        let vmax = bforh.max();
        println!(" - highest of '{}M' calls/s -> bench #{}", 
            (vmax.1 / 1_000_000.0).round(), vmax.0);
        
    println!("Swarm 'heap' update results:");
        println!(" - average of '{}M' calls/s", (bupdh.avg() / 1_000_000.0).round());
        println!(" - average speed was '{}%' of plain vector speed", 
            ((bupdh.avg() / bvec.avg()) * 100_000.0).round() / 1_000.0);
        let vmin = bupdh.min();
        println!(" - lowest of '{}M' calls/s -> becnh #{}", 
            (vmin.1 / 1_000_000.0).round(), vmin.0);
        let vmax = bupdh.max();
        println!(" - highest of '{}M' calls/s -> becnh #{}", 
            (vmax.1 / 1_000_000.0).round(), vmax.0);

    println!("Swarm 'heap' update-ctl results:");
        println!(" - average of '{}M' calls/s", (bupch.avg() / 1_000_000.0).round());
        println!(" - average speed was '{}%' of plain vector speed", 
            ((bupch.avg() / bvec.avg()) * 100_000.0).round() / 1_000.0);

        let vmin = bupch.min();
        println!(" - lowest of '{}M' calls/s -> becnh #{}", 
            (vmin.1 / 1_000_000.0).round(), vmin.0);
        let vmax = bupch.max();
        println!(" - highest of '{}M' calls/s -> becnh #{}", 
            (vmax.1 / 1_000_000.0).round(), vmax.0);
    
    println!("Swarm 'stack' foreach results:");
        println!(" - average of '{}M' calls/s", (bfors.avg() / 1_000_000.0).round());
        println!(" - av. speed was '{}%' of plain vector speed", 
            ((bfors.avg() / bvec.avg()) * 100_000.0).round() / 1_000.0);
        let vmin = bfors.min();
        println!(" - lowest of '{}M' calls/s -> bench #{}", 
            (vmin.1 / 1_000_000.0).round(), vmin.0);
        let vmax = bfors.max();
        println!(" - highest of '{}M' calls/s -> bench #{}", 
            (vmax.1 / 1_000_000.0).round(), vmax.0);
    
    println!("Swarm 'stack' update results:");
        println!(" - average of '{}M' calls/s", (bupds.avg() / 1_000_000.0).round());
        println!(" - average speed was '{}%' of plain vector speed", 
            ((bupds.avg() / bvec.avg()) * 100_000.0).round() / 1_000.0);
        let vmin = bupds.min();
        println!(" - lowest of '{}M' calls/s -> becnh #{}", 
            (vmin.1 / 1_000_000.0).round(), vmin.0);
        let vmax = bupds.max();
        println!(" - highest of '{}M' calls/s -> becnh #{}", 
            (vmax.1 / 1_000_000.0).round(), vmax.0);

    println!("Swarm 'stack' update-ctl results:");
        println!(" - average of '{}M' calls/s", (bupcs.avg() / 1_000_000.0).round());
        println!(" - average speed was '{}%' of plain vector speed", 
            ((bupcs.avg() / bvec.avg()) * 100_000.0).round() / 1_000.0);
        let vmin = bupcs.min();
        println!(" - lowest of '{}M' calls/s -> becnh #{}", 
            (vmin.1 / 1_000_000.0).round(), vmin.0);
        let vmax = bupcs.max();
        println!(" - highest of '{}M' calls/s -> becnh #{}", 
            (vmax.1 / 1_000_000.0).round(), vmax.0);

}

fn bench_with_objects(run_id: &mut usize) -> (Bench, Bench, Bench, Bench, Bench, Bench, Bench) {
    
    let (v_spd1, fh_spd1, uh_spd1, ch_spd1, fs_spd1, us_spd1, cs_spd1) = bench_with(run_id, 1);
    let (v_spd2, fh_spd2, uh_spd2, ch_spd2, fs_spd2, us_spd2, cs_spd2) = bench_with(run_id, 10);
    let (v_spd3, fh_spd3, uh_spd3, ch_spd3, fs_spd3, us_spd3, cs_spd3) = bench_with(run_id, 100);
    let (v_spd4, fh_spd4, uh_spd4, ch_spd4, fs_spd4, us_spd4, cs_spd4) = bench_with(run_id, 1_000);
    //let (v_spd5, fh_spd5, uh_spd5, fs_spd5, us_spd5) = bench_with(run_id, 10_000);
    //let (v_spd6, fh_spd6, uh_spd6, fs_spd6, us_spd6) = bench_with(run_id, 100_000);
    //let (v_spd7, fh_spd7, uh_spd7, fs_spd7, us_spd7) = bench_with(run_id, 1_000_000);

    println!("--");
    (   
        Bench (vec![v_spd1, v_spd2, v_spd3, v_spd4]),//, v_spd5, v_spd6, v_spd7]),
        Bench (vec![fh_spd1, fh_spd2, fh_spd3, fh_spd4]),//, fh_spd5, fh_spd6, fh_spd7]),
        Bench (vec![uh_spd1, uh_spd2, uh_spd3, uh_spd4]),//, uh_spd5, uh_spd6, uh_spd7]),
        Bench (vec![ch_spd1, ch_spd2, ch_spd3, ch_spd4]),//, uh_spd5, uh_spd6, uh_spd7]),
        Bench (vec![fs_spd1, fs_spd2, fs_spd3, fs_spd4]),//, fs_spd5, fs_spd6, fs_spd7]),
        Bench (vec![us_spd1, us_spd2, us_spd3, us_spd4]),//, us_spd5, us_spd6, us_spd7]),
        Bench (vec![cs_spd1, cs_spd2, cs_spd3, cs_spd4]),//, us_spd5, us_spd6, us_spd7]),
    )
}

type Speed = (usize, f64);

fn bench_with(run_id: &mut usize, objects: usize) -> (Speed, Speed, Speed, Speed, Speed, Speed, Speed) {
    let fn_avg = |x: f64, vec: f64| (100.0 * x / vec).round();

    std::thread::sleep(std::time::Duration::from_millis(500));
    
    let vec_spd = vec_heap_bencher(run_id, objects);
    println!(" ..result was => '{}M' call/s", (vec_spd.1 / 1_000_000.0).round());

    let for_h_spd = for_heap_bencher(run_id, objects);
    println!(" ..result was => '{}M' call/s ({}%)", 
        (for_h_spd.1 / 1_000_000.0).round(),
        fn_avg(for_h_spd.1, vec_spd.1)
    );
    let upd_h_spd = update_heap_bencher(run_id, objects);
    println!(" ..result was => '{}M' call/s ({}%)", 
        (upd_h_spd.1 / 1_000_000.0).round(), 
        fn_avg(upd_h_spd.1, vec_spd.1)
    ); 
    let uct_h_spd = update_ctl_heap_bencher(run_id, objects);
    println!(" ..result was => '{}M' call/s ({}%)", 
        (uct_h_spd.1 / 1_000_000.0).round(), 
        fn_avg(uct_h_spd.1, vec_spd.1)
    ); 
    let for_s_spd = for_stack_bencher(run_id, objects);
    println!(" ..result was => '{}M' call/s ({}%)", 
        (for_s_spd.1 / 1_000_000.0).round(),
        fn_avg(for_s_spd.1, vec_spd.1)
    );
    let upd_s_spd = update_stack_bencher(run_id, objects);
    println!(" ..result was => '{}M' call/s ({}%)", 
        (upd_s_spd.1 / 1_000_000.0).round(), 
        fn_avg(upd_s_spd.1, vec_spd.1)
    ); 
    let uct_s_spd = update_ctl_stack_bencher(run_id, objects);
    println!(" ..result was => '{}M' call/s ({}%)", 
        (uct_s_spd.1 / 1_000_000.0).round(), 
        fn_avg(uct_s_spd.1, vec_spd.1)
    );

    (vec_spd, for_h_spd, upd_h_spd, uct_h_spd, for_s_spd, upd_s_spd, uct_s_spd)
}

struct Bench(Vec<Speed>);

impl Bench {
    pub fn merge(mut self, other: Bench) -> Self {
        for v in other.0 {
            self.0.push(v);
        }
        self
    }

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
    println!("{}: Running Standard Vec bench for {} objects",id, amount);
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
    println!("{}: Running Heap Swarm foreach bench for {} objects", id, amount);
    // get swarm ecs system speed
    let mut swarm = HeapSwarm::<Minion>::new(amount);
    for _e in 0..amount { swarm.spawn(); }

    let now = std::time::SystemTime::now();
    for _j in 0..NUM_SAMPLES/amount { 
        swarm.for_each(|obj| {
            obj.times_summoned += 1;
        });
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = elapsed_res.unwrap().as_secs_f64();
    let swarm_speed = (swarm.get_mut(&0).times_summoned * amount as u128) as f64 / swarm_time;

    (id.clone(), swarm_speed)
}

fn update_heap_bencher(id: &mut usize, amount: usize) -> Speed {
    *id += 1;
    println!("{}: Running Heap Swarm Update bench for {} objects", id, amount);
    // get swarm ecs system speed
    let mut swarm = HeapSwarm::<Minion>::new(amount);
    for _e in 0..amount { 
        let spawn = swarm.spawn().unwrap();
        swarm.get_mut(&spawn).summon = Some(Summoning::default());
    }

    let now = std::time::SystemTime::now();
    for _j in 0..NUM_SAMPLES/amount { 
        swarm.update(|ptr, swarm| {
            swarm[*ptr].times_summoned += 1;
        });
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = elapsed_res.unwrap().as_secs_f64();
    let swarm_speed = (swarm.get_mut(&0).times_summoned * amount as u128) as f64 / swarm_time;

    (id.clone(), swarm_speed)
}

fn update_ctl_heap_bencher(id: &mut usize, amount: usize) -> Speed {
    *id += 1;
    println!("{}: Running Heap Swarm UpdateCTL bench for {} objects", id, amount);
    // get swarm ecs system speed
    let mut swarm = HeapSwarm::<Minion>::new(amount);
    for _e in 0..amount { 
        let spawn = swarm.spawn().unwrap();
        swarm.get_mut(&spawn).summon = Some(Summoning::default());
    }

    let now = std::time::SystemTime::now();
    for _j in 0..NUM_SAMPLES/amount { 
        swarm.update_ctl(|spawn, swarm| {
            swarm.get_mut(spawn).times_summoned += 1;
        });
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = elapsed_res.unwrap().as_secs_f64();
    let swarm_speed = (swarm.get_mut(&0).times_summoned * amount as u128) as f64 / swarm_time;

    (id.clone(), swarm_speed)
}



fn for_stack_bencher(id: &mut usize, amount: usize) -> Speed {
    *id += 1;
    println!("{}: Running Stack Swarm foreach bench for {} objects", id, amount);
    // get swarm ecs system speed
    let mut swarm = StackSwarm::<Minion>::new();
    let spawns = match amount { a if a <= 1_000 => a, _ => 1_000, };

    for _e in 0..spawns { swarm.spawn(); }

    let now = std::time::SystemTime::now();
    for _j in 0..NUM_SAMPLES/amount { 
        swarm.for_each(|obj| {
            obj.times_summoned += 1;
        });
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = elapsed_res.unwrap().as_secs_f64();
    let swarm_speed = (swarm.get_mut(&0).times_summoned * amount as u128) as f64 / swarm_time;

    (id.clone(), swarm_speed)
}

fn update_stack_bencher(id: &mut usize, amount: usize) -> Speed {
    *id += 1;
    println!("{}: Running Stack Swarm Update bench for {} objects", id, amount);
    // get swarm ecs system speed
    let mut swarm = StackSwarm::<Minion>::new();
    let spawns = match amount { a if a <= a => a, _ => 1_000, };

    for _e in 0..spawns { 
        let spawn = swarm.spawn().unwrap();
        swarm.get_mut(&spawn).summon = Some(Summoning::default());
    }

    let now = std::time::SystemTime::now();
    for _i in 0..NUM_SAMPLES/amount {
        swarm.update(|ptr, swarm| {
            swarm[*ptr].times_summoned += 1;
        });
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = elapsed_res.unwrap().as_secs_f64();
    let swarm_speed = (swarm.get_mut(&0).times_summoned * spawns as u128) as f64 / swarm_time;

    (id.clone(), swarm_speed)
}

fn update_ctl_stack_bencher(id: &mut usize, amount: usize) -> Speed {
    *id += 1;
    println!("{}: Running Stack Swarm UpdateCTL bench for {} objects", id, amount);
    // get swarm ecs system speed
    let mut swarm = StackSwarm::<Minion>::new();
    let spawns = match amount { a if a <= a => a, _ => 1_000, };

    for _e in 0..spawns { 
        let spawn = swarm.spawn().unwrap();
        swarm.get_mut(&spawn).summon = Some(Summoning::default());
    }

    let now = std::time::SystemTime::now();
    for _i in 0..NUM_SAMPLES/amount {
        swarm.update_ctl(|spawn, swarm| {
            swarm.get_mut(spawn).times_summoned += 1;
        });
    }
    let elapsed_res = now.elapsed();

    // swarm test results
    let swarm_time = elapsed_res.unwrap().as_secs_f64();
    let swarm_speed = (swarm.get_mut(&0).times_summoned * spawns as u128) as f64 / swarm_time;

    (id.clone(), swarm_speed)
}

