use super::*;

// POOL 128

const CAPACITY: usize = 128;

/// Sized pool object with a max capacity of 128 items.
pub struct SizedPool128<ItemType> { 
    items: [Option<ItemType>; CAPACITY], 
    order: [usize; CAPACITY], 
    count: usize,
}

impl<ItemType> SizedPool128<ItemType>
where ItemType: Copy + PartialEq
{
    #[allow(unused)]
    /// Creates a new pool.
    pub fn new() -> Self {
        let mut order = [0; CAPACITY]; 
        for i in 0..CAPACITY { order[i] = i; }
        SizedPool128 {
            items: [None; CAPACITY],
            order,
            count: 0,
        }
    }

    /// Returns a new SizedPool that contains copies of all of
    /// the objects that where passed through the items array 
    /// list parameter.
    ///
    /// All items that exceed the maximum capacity of this 
    /// SizedPool are negated and a full item list containing the 
    /// first part of the items property is returned.
    /// 
    /// # Example
    /// ```
    /// use swarm_pool::tools::sized_pool as pool;
    /// use swarm_pool::tools::sized_pool::SizedPool128;
    /// 
    /// let empty_pool = SizedPool128::<usize>::new();
    /// assert_eq!(pool::count(&empty_pool), &0);
    ///
    /// let mut pool16 = SizedPool128::<usize>::from_slice(&[
    ///     1,2,3,4,5,6,7,8,9,10
    /// ]);
    ///
    /// assert_eq!(pool::count(&pool16), &10);
    /// assert_eq!(pool::get_ref(&pool16, 0), &Some(1));
    /// assert_eq!(pool::get_ref(&pool16, 9), &Some(10));
    /// 
    /// // If an item in the pool is unused it returns None
    /// // a request index larger than 15 will panic!
    /// assert_eq!(pool::get_ref(&pool16, 15), &None);    
    /// ```
    pub fn from_slice(items: &[ItemType]) -> SizedPool128<ItemType> {
        let mut new_pool = SizedPool128::<ItemType>::new();
        
        for i in 0..items.len() {
            if i >= CAPACITY {
                break;
            } else {
                push(&mut new_pool, items[i]);
            } 
        }
        new_pool
    }

    #[allow(unused)]
    pub(crate) fn get_mut(&mut self, position: &usize) -> &mut Option<ItemType> {
        &mut self.items[self.order[*position]]
    }

    #[allow(unused)]
    pub(crate) fn get_ref(&self, position: &usize) -> &Option<ItemType> {
        &self.items[self.order[*position]]
    }
}

impl<ItemType> StackPool<ItemType> for SizedPool128<ItemType> {
    fn count(&self) -> &usize { 
        &self.count 
    }

    fn count_mut(&mut self) -> &mut usize { 
        &mut self.count 
    }

    fn ref_at(&self, item_index: &usize) -> &Option<ItemType> {
        &self.items[*item_index]
    }

    fn ref_sorted(&self, ord_index: &usize) -> &Option<ItemType> {
        &self.items[self.order[*ord_index]]
    }
    
    fn item_at(&mut self, item_index: &usize) -> &mut Option<ItemType> {
        &mut self.items[*item_index]
    }
    
    fn item_last(&mut self) -> &mut Option<ItemType> {
        &mut self.items[self.count]
    }

    fn item_sorted(&mut self, ord_index: &usize) -> &mut Option<ItemType> {
        &mut self.items[self.order[*ord_index]]
    }
    
    fn order_at(&mut self, ord_index: &usize) -> &mut usize {
        &mut self.order[*ord_index]
    }
}