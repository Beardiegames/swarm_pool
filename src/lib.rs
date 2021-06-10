#[allow(dead_code)]

use std::alloc::{alloc, dealloc, Layout, LayoutError};
use std::marker::PhantomData;

mod tests;

#[derive(Debug)]
pub enum SwarmError {
    MemoryLayoutFailure,
}

pub type SpawnId = usize;

pub trait System {
    type Entity;
    fn update(&mut self, entity: &mut Self::Entity);
}

pub struct Swarm<T: Default + Copy> {
    content: Vec<T>,
    //phan: PhantomData<T>,
    //ptr: *mut u8,
    len: usize,
    max: usize,
    //system: fn(&mut T),
    system: Box<dyn System<Entity = T>>,
}

impl<T: Default + Copy> Swarm<T> {

    pub fn new(size: usize, system: Box<dyn System<Entity = T>>) -> Result<Self, SwarmError> {

        //let fmt: Layout;
        // let ptr: *mut u8;

        // unsafe {
        //     let set_mem = Layout::array::<T>(1_000_000)
        //         .map_err(|_e| SwarmError::MemoryLayoutFailure)?;
        //     ptr = alloc(set_mem);
        //     *(ptr as *mut [T; 1_000_000]) = [T::default(); 1_000_000];
        // }
        

        Ok (Swarm { 
            //phan: PhantomData,
            //ptr,
            content: vec![T::default(); size],
            len: 0,
            max: size,
            system,
        })
    }

    pub fn spawn(&mut self) -> Option<SpawnId> {
        if self.len < self.max {
            self.len += 1;
            Some(self.len)
        } else {
            None
        }
    }

    pub fn kill(&mut self, at_index: usize) {
        self.content[at_index] = self.content[self.len-2];  // swap to back
        self.len -= 1;                                      // decrement size
    }

    pub fn for_each(&mut self) {//}, update: fn(&mut T)) {
        //unsafe {
            // let mut i = 0;
            // let dat = &mut *(self.ptr as *mut [T; 1_000_000]);
            // while i < self.len as usize {
            //     update(&mut dat[i]);
            //     i += 1;
            // }
            for i in 0..self.len{
                //update(&mut self.content[i]);
                //(self.system)(&mut self.content[i]);
                self.system.update(&mut self.content[i]);
            }
        //}
    }

    pub fn get_mut(&mut self, id: SpawnId) -> &mut T {
        // unsafe {
        //     &mut (*(self.ptr as *mut [T; 1_000_000]))[n]
        // }
        &mut self.content[id]
    }
}


// pub fn for_each<T: Default>(swarm: &mut Swarm<T>, update: fn(&mut T)) {
//     unsafe {
//         let mut i = 0;
//         while i < swarm.len {
//             update(&mut *swarm.ptr.offset(i));
//             i += 1;
//         }
//     }
// }