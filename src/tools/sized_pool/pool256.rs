use super::StackPool;

// POOL 256

const CAPACITY: usize = 256;

/// Sized pool object with a max capacity of 256 items.
pub struct SizedPool256<ItemType> { 
    items: [Option<ItemType>; CAPACITY], 
    order: [usize; CAPACITY], 
    count: usize,
}

impl<ItemType> SizedPool256<ItemType>
where ItemType: Copy + PartialEq
{
    #[allow(unused)]
    /// Creates a new pool.
    pub fn new() -> Self {
        let mut order = [0; CAPACITY]; 
        for i in 0..CAPACITY { order[i] = i; }
        SizedPool256 {
            items: [None; CAPACITY],
            order,
            count: 0,
        }
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

impl<ItemType> StackPool<ItemType> for SizedPool256<ItemType> {
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