
mod tests;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Spawn(usize);


#[allow(dead_code)]
pub struct Pool<T: Default>(Vec<T>);

impl<T: Default> Pool<T> {

    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        let mut pool = Vec::<T>::new();
        pool.resize_with(size, T::default);

        Pool (pool)
    }

    #[allow(dead_code)]
    pub fn get_body(&mut self, spawn: &Spawn) -> &mut T {
        &mut self.0[spawn.0]
    }
}

pub struct Swarm<T: Default>{
    pool: Pool<T>,
    spawns: Vec<Spawn>,
    free: Vec<Spawn>,
}

impl<T: Default> Swarm<T> {

    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        let spawns = Vec::<Spawn>::with_capacity(size);
        let mut free = Vec::<Spawn>::with_capacity(size);
        for i in 0..size { free.push(Spawn (i)); }

        Swarm { pool: Pool::new(size), spawns, free, }
    }

    #[allow(dead_code)]
    pub fn get_body(&mut self, spawn: &Spawn) -> &mut T {
        self.pool.get_body(spawn)
    }

    #[allow(dead_code)]
    pub fn spawn(&mut self) -> Option<Spawn> {
        if let Some(spawn) = self.free.pop() {
            self.spawns.push(spawn.clone());
            Some(spawn)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    // NOTE: killing becomes slower as number of spawns increases
    pub fn kill(&mut self, spawn: Spawn) {
        if let Some(index) = self.spawns.iter().position(|x| x.0 == spawn.0) {
            self.free.push(self.spawns.remove(index));
        }
    }

    pub fn for_each(&mut self, update: fn(&Spawn, &mut Pool<T>)) {
        for spawn in &self.spawns {
            update(&spawn, &mut self.pool);
        }
    }
}
