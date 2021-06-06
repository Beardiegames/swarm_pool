#[allow(dead_code)]

// use std::alloc::{alloc, dealloc, Layout};
// use std::marker::PhantomData;
mod tests;


pub struct Swarm<T: Default> {
    dat: Vec<T>,
    arr: [*mut T; 1],
    len: usize,
}

impl<T: Default> Swarm<T> {

    pub fn new() -> Self {
        let mut dat = vec![T::default()];
        let dat_ref: *mut T = &mut dat [0];
        let arr: [*mut T; 1] = [dat_ref];

        Swarm { arr, len: 1, dat }
    }

    pub fn spawn(&mut self) {
        if self.len < self.dat.len() {
            self.len += 1;
        }
    }

    pub fn kill(&mut self, at_index: usize) {
        self.arr[at_index] = self.arr[self.len-2];  // swap to back
        self.len -= 1;                              // decrement size
    }
}


pub fn for_each<T: Default>(ref_arr: &mut Swarm<T>, update: fn(&mut T)) {
    unsafe {
        let mut i = 0;
        while i < ref_arr.len {
            update(&mut *ref_arr.arr[i]);
            i += 1;
        }
    }
}