# swarm_pool
An object pooling system for Rust, optimized for perfomance.

## Contents
1. [Changelog](#changelog)
2. [Using the swarm pool](#usage)
3. [Benchmarks](#benchmark)
4. [Using Swam for ECS](#ecs)

<h2 id="changelog">Changelog</h2>

 - **version 0.1.8**:
    - Added `kill_all()` methode to `Swarm`. Kill all removes all spawns at once, can only be called from Swarm itself, NOT from SwarmControl.
    - Added Benchmark and ECS documentation to this README.
 - **version 0.1.7**: 
    - Removed `Copy` dependencies, PoolObjects do no longer rely on the implementation of the `Copy` trait.
    - Implemented the `Clone` & `Default` traits for `Spawn`. Spawn.clone() behaves in the same way as Spawn.mirror() does. The These where implemented so that PoolObjects can hold Spawn reference objects as field properties. Default on Spawn should not be used and is only there to make the above possible.
 - **version 0.1.4**: 
    - Added iterators `find()`, `for_while()`, `for_each()`, `for_all()` & `enumerate()` to `SwarmComtrol`, these makes it possible to iterate over all spawns inside the update() loop.


<h2 id="usage"> Using the swarm pool</h2>

The pooling system manages object instances of a cutom type,
and provides update loops to iterate over them.

In order to create a new swarm pool, you need to define what your `pool object` and `swarm properties` types
are going to look like. Your `pool object` must at leas implement the Default, Copy and Clone traits 
from the standard library. The `swarm properties`, on the other hand, does not depend on any traits.

### Basic swarm setup example
```rust
extern crate swarm_pool;
use swarm_pool::Swarm;

// create an object you want to pool
#[derive(Default, Clone)]     
pub struct MyPoolObject {           
    pub name: &static str,
    pub value: usize,
}

// create properties you want to share with pooled objects
pub struct MySwarmProperties;

fn main() {
    let swarm = Swarm::<MyPoolObject, MySwarmProperties>::new(10, MySwarmProperties);
    assert!(swarm.capacity() == 10);
}
```

The swarm is now ready to be used. First of all we need to Spawn new pool instances. In reality all
objects in the pool are allready created and are waiting to be used. This means that all objects (
from 0 up to, but not including, the maximum capacity) can be accessed through the fetch() methode.
The difference between spawned and non-spawned pool objects is that spawned object are included in all
of the Swarm pools iterator methodes and non-spawned object are not.

### Spawning and looping
```rust
let mut swarm = Swarm::<MyPoolObject, _>::new(10, ());
let spawn1 = swarm.spawn().unwrap();
let spawn2 = swarm.spawn().unwrap();
  
assert_eq!(swarm.fetch_ref(&spawn1).value, 0);
assert_eq!(swarm.fetch_ref(&spawn2).value, 0);

swarm.for_each(|obj| {
    obj.value = 42;
});

assert_eq!(swarm.fetch_ref(&spawn1).value, 42);
assert_eq!(swarm.fetch_ref(&spawn2).value, 42);
```

The real power of this library is not just looping through a few object instances, it is controlling and cross referencing them.
There are 2 powerful methodes that can be used to do so: `Swarm.for_all()` and `Swarm.update()`.
Both have their advantages and disadvantages, `for_all` loop is fast (equal to a standard vec for loop) but cannot spawn nor kill
pool objects, `update` is easy to use, gives full control, but is slow (less than half the speed).

### Cross referencing using for_all & update
```rust
// change properties to contain references to our spawned pool objects
pub struct MySwarmProperties { 
    john: Option<Spawn>, 
    cristy: Option<Spawn>,
}

let properties = MySwarmProperties { john: None, cristy: None };

let mut swarm = Swarm::<MyPoolObject, MySwarmProperties>::new(10, properties);
let s_john = swarm.spawn().unwrap();
let s_cristy = swarm.spawn().unwrap();

swarm.properties.john = Some(s_john.mirror());
swarm.properties.cristy = Some(s_cristy.mirror());

swarm.fetch(&s_john).name = "John";
swarm.fetch(&s_cristy).name = "Cristy";

// using the for_all methode
swarm.for_all(|target, list, props| {

    // john tells critsy to have a value of 2
    if list[*target].name == "John" { 
        if let Some(cristy) = &props.cristy {
            list[cristy.pos()].value = 2; 
        }
    }
    // cristy tells john to have a value of 1
    if list[*target].name == "Cristy" { 
        if let Some(john) = &props.john {
            list[john.pos()].value = 1; 
        }
    }
});

assert_eq!(swarm.fetch_ref(&s_john).value, 1);
assert_eq!(swarm.fetch_ref(&s_cristy).value, 2);

// using the update methode
swarm.update(|ctl| {
    let name = ctl.target().name;
    let cristy = ctl.properties.cristy.as_ref().unwrap().mirror();
    let john = ctl.properties.john.as_ref().unwrap().mirror();

    // john tells critsy to have a value of 4
    if name == "John" { 
        ctl.fetch(&cristy).value = 4; 
    }
    // cristy tells john to have a value of 5
    if name == "Cristy" { 
        ctl.fetch(&john).value = 5; 
    }
});

assert_eq!(swarm.fetch_ref(&s_john).value, 5);
assert_eq!(swarm.fetch_ref(&s_cristy).value, 4);
```

There are many more functionalities included in the Swarm and SwarmControl types. 
The documentation on the examples above or other functionalities this library provides are more in depth
and should be read, for writing them out was a lot of work ;)

<h2 id="benchmark"> Benchmarking</h2>

Benchmark performance is tested using a standard Vector as baseline. This standard vector is populated by the same object type and a standard for loop is used to iterate over the elements. Every object, when called, adds one to its value property. 


### 0.1.8 - Benchmark results on my i7 laptop:

```
1. Standard Vec bench for 1 object(s).. 814M calls/s @ 814M upd/s
2. Swarm.for_each() bench with 1 object(s).. 825M calls/s(101%) @ 825M upd/s
3. Swarm.for_all() bench with 1 object(s).. 677M calls/s(83%) @ 677M upd/s
4. Swarm.update() bench with 1 object(s).. 530M calls/s(65%) @ 530M upd/s
5. Standard Vec bench for 10 object(s).. 1568M calls/s @ 156.8M upd/s
6. Swarm.for_each() bench with 10 object(s).. 1596M calls/s(102%) @ 159.6M upd/s
7. Swarm.for_all() bench with 10 object(s).. 1049M calls/s(67%) @ 104.9M upd/s
8. Swarm.update() bench with 10 object(s).. 695M calls/s(44%) @ 69.5M upd/s
9. Standard Vec bench for 100 object(s).. 1550M calls/s @ 15.5M upd/s
10. Swarm.for_each() bench with 100 object(s).. 1419M calls/s(92%) @ 14.19M upd/s
11. Swarm.for_all() bench with 100 object(s).. 1051M calls/s(68%) @ 10.51M upd/s
12. Swarm.update() bench with 100 object(s).. 676M calls/s(44%) @ 6.76M upd/s
13. Standard Vec bench for 1000 object(s).. 1234M calls/s @ 1.234M upd/s
14. Swarm.for_each() bench with 1000 object(s).. 1231M calls/s(100%) @ 1.231M upd/s
15. Swarm.for_all() bench with 1000 object(s).. 1058M calls/s(86%) @ 1.058M upd/s
16. Swarm.update() bench with 1000 object(s).. 678M calls/s(55%) @ 0.678M upd/s

# RESULTS TOTAL
* Plain vec results:
  - average of '1292M' calls/s
  - lowest of '814M' calls/s (bench #1)
  - highest of '1568M' calls/s (becnh #5)
* swarm.for_each() results:
  - average of '1268M' calls/s
  - average speed was '98.168%' of plain vector speed
  - lowest of '825M' calls/s (becnh #2)
  - highest of '1596M' calls/s (becnh #6)
* swarm.for_all() results:
  - average of '959M' calls/s
  - average speed was '74.242%' of plain vector speed
  - lowest of '677M' calls/s (becnh #3)
  - highest of '1058M' calls/s (becnh #15)
* swarm.update() results:
  - average of '645M' calls/s
  - average speed was '49.916%' of plain vector speed
  - lowest of '530M' calls/s (becnh #4)
  - highest of '695M' calls/s (becnh #8)
```
The result varies per test run, but the average speed percentage always seems somewhat stable. In other words, the benchmark results give a general idea what the scope of your ballpark is going to be when using the swarm_pool library.


<h2 id="ecs">Using Swam for ECS</h2>

For those whome would like to dive into ECS engine developement, the question: _"Can one take a swarm_pool and turn it into an ECS engine?"_, the answer is: _"Yes, one could"_. 

I've tried different ways of using the Swarm pool to create an ECS engine. It turns out that, when using the swarm pool, the `Option<>` approach seems to be faster than the classic `bitflag` strategy. In other words, wrapping your Components in a standard Option, and perform a per object test if that object has Some(component) for the system that is looping through the swarm. 

### ECS Example
```rust
#[derive(Default, Clone, Debug, PartialEq)] struct Image(bool);
#[derive(Default, Clone, Debug, PartialEq)] struct Position(f32, f32);
#[derive(Default, Clone, Debug, PartialEq)] struct Speed(f32);

#[derive(Default, Clone, Debug)]
pub struct Entity {
    name: &'static str,
    
    image_component: Option<Image>,
    position_component: Option<Position>,
    speed_component: Option<Speed>,
}

fn main() {
    let mut swarm = Swarm::<Entity, _>::new(10, ());
    
    let building = swarm.spawn().unwrap();
    {
        let entity = swarm.fetch(&building);
            entity.name = "Sixteenth Chapel";
            entity.image_component = Some(Image(false));
            entity.position_component = Some(Position(3.0, 5.0));
            entity.speed_component = None;
    }

    let truck = swarm.spawn().unwrap();
    {
        let entity = swarm.fetch(&truck);
            entity.name = "Cargo truck";
            entity.image_component = Some(Image(false));
            entity.position_component = Some(Position(8.0, 6.0));
            entity.speed_component = Some(Speed(1.0));
    }

    assert_eq!(swarm.fetch_ref(&building).position_component, Some(Position(3.0, 5.0)));
    assert_eq!(swarm.fetch_ref(&building).image_component, Some(Image(false)));

    assert_eq!(swarm.fetch_ref(&truck).position_component, Some(Position(8.0, 6.0)));
    assert_eq!(swarm.fetch_ref(&truck).image_component, Some(Image(false)));

    // # MOVE SYSTEM
    swarm.for_all(|tar, pool, props|{
        if let ( 
            Some(position_component), 
            Some(speed_component)
        ) = (
            &mut pool[*tar].position_component, 
            &mut pool[*tar].speed_component 
        ){
            position_component.0 += speed_component.0;
        }
    });

    // Only the truck moves from position 8.0 to 9.0, the building does not 
    // move because it does not have a Speed component which is required 
    // by the move system.
    assert_eq!(swarm.fetch_ref(&building).position_component, Some(Position(3.0, 5.0)));
    assert_eq!(swarm.fetch_ref(&truck).position_component, Some(Position(9.0, 6.0)));

    // # DRAW SYSTEM
    swarm.for_all(|tar, pool, props|{
        if let ( 
            Some(position_component), 
            Some(image_component)
        ) = (
            &mut pool[*tar].position_component, 
            &mut pool[*tar].image_component 
        ){
            image_component.0 = true;
        }
    });

    // Both building and truck should be updated because both have a Position 
    // and an Image component, which are required by the draw system.
    assert_eq!(swarm.fetch_ref(&building).image_component, Some(Image(true)));
    assert_eq!(swarm.fetch_ref(&truck).image_component, Some(Image(true)));
}
```

You might have noticed the use of `for_all()` instead of `update()`. Update is slower and therefore, when building your own engine, it is preferable to use for_all. I would advice to only use update if you need to kill/spawn during update or you need to loop through all objects while looping through all objects. 
