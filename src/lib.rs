
mod tests;

pub enum SwarmResult {
    SpawnFailed,
}

#[allow(dead_code)]
type Behaviour<T> = fn(&mut T);

#[allow(dead_code)]
type BehaviourId = usize;

#[allow(dead_code)]
type SpawnId = usize;

#[allow(dead_code)]
struct Swarm<T: Default> {
    behaviours: Vec<Box<Behaviour<T>>>,
    assignments: Vec<Vec<SpawnId>>,
    pool: Vec<T>,
    spawned_minions: Vec<SpawnId>,
    free_minions: Vec<SpawnId>,
}

impl<T: Default> Swarm<T> {

    #[allow(dead_code)]
    fn new(size: usize) -> Self {
        let mut pool = Vec::<T>::new();
        pool.resize_with(size, T::default);

        let spawned_minions = Vec::<SpawnId>::with_capacity(size);
        let mut free_minions = Vec::<SpawnId>::with_capacity(size);

        for i in 0..size { free_minions.push(i); }

        Swarm { 
            behaviours: Vec::new(),
            assignments: Vec::new(),
            pool,
            spawned_minions,
            free_minions,
        }
    }

    #[allow(dead_code)]
    fn behaviour(&mut self, as_closure: Behaviour<T>) -> BehaviourId {
        self.behaviours.push(Box::new(as_closure));
        self.assignments.push(Vec::new());
        self.behaviours.len() - 1
    }

    #[allow(dead_code)]
    fn assign(&mut self, behaviour: &BehaviourId, to: &SpawnId) {
        self.assignments[*behaviour].push(*to);
    }

    #[allow(dead_code)]
    fn is_assigned(&mut self, behaviour: &BehaviourId, to: &SpawnId) -> bool {
        self.assignments[*behaviour].contains(&to)
    }

    #[allow(dead_code)]
    fn trigger_all(&mut self, behaviour: &BehaviourId) {
        for spawn in &self.assignments[*behaviour] {
            self.behaviours[*behaviour](&mut self.pool[*spawn])
        }
    }

    #[allow(dead_code)]
    // NOTE: trigger also works if not assigned to behaviour -> speed optimization
    fn trigger(&mut self, behaviour: &BehaviourId, spawn: &SpawnId) {
        self.behaviours[*behaviour](&mut self.pool[*spawn])
    }

    #[allow(dead_code)]
    // NOTE: slow when many spawns have been assigned to the requested behaviour
    fn revoke(&mut self, behaviour: &BehaviourId, from: &SpawnId) {
        if let Some(i) = self.assignments[*behaviour].iter().position(|x| x == from) {
            self.assignments[*behaviour].remove(i);
        }
    }

    #[allow(dead_code)]
    fn minion(&mut self, spawn: &SpawnId) -> &mut T {
        &mut self.pool[*spawn]
    }

    #[allow(dead_code)]
    fn spawn(&mut self) -> Option<SpawnId> {
        if let Some(spawn) = self.free_minions.pop() {
            self.spawned_minions.push(spawn);
            Some(spawn)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    // NOTE: killing becomes slower as number of spawns increases
    fn kill(&mut self, spawn: SpawnId) {
        if let Some(i) = self.spawned_minions.iter().position(|x| x == &spawn) {
            self.spawned_minions.remove(i);
            self.free_minions.push(spawn);
        }
    }
}
