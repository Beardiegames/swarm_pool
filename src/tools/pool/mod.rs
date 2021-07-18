mod pool16; 
mod pool32;
mod pool64;
mod pool128;
mod pool256;

#[allow(unused_imports)] pub use pool16::StackPool16;
#[allow(unused_imports)] pub use pool32::StackPool32;
#[allow(unused_imports)] pub use pool64::StackPool64;
#[allow(unused_imports)] pub use pool128::StackPool128;
#[allow(unused_imports)] pub use pool256::StackPool256;

pub trait StackPool <ItemType> {
    fn count(&self) -> &usize;
    fn count_mut(&mut self) -> &mut usize;

    fn ref_at(&self, item_index: &usize) -> &Option<ItemType>;
    fn ref_sorted(&self, ord_index: &usize) -> &Option<ItemType>;

    fn item_at(&mut self, item_index: &usize) -> &mut Option<ItemType>;
    fn item_last(&mut self) -> &mut Option<ItemType>;
    fn item_sorted(&mut self, ord_index: &usize) -> &mut Option<ItemType>;
    fn order_at(&mut self, ord_index: &usize) -> &mut usize;
}

#[allow(unused)]
/// Loop through all pushed items and edit them using a callback handler
/// 
/// Example:
/// ```
/// use swarm::tools::pool::StackPool16;
/// use swarm::tools::pool;
/// 
/// fn main() {
///     let mut pool16: StackPool16<u8> = StackPool16::new();
///     pool::push(&mut pool16, 1);
///     pool::push(&mut pool16, 2);
///     pool::push(&mut pool16, 3);
///     
///     pool::for_each(&mut pool16, |item| *item *= 2 );
/// 
///     assert_eq!(pool16.get_ref(&0), &Some(2));
///     assert_eq!(pool16.get_ref(&1), &Some(4));
///     assert_eq!(pool16.get_ref(&2), &Some(6));
/// }
/// ```
pub fn for_each<ItemType, Pool, Callback>(pool: &mut Pool, handler: Callback) 
where   ItemType: Copy + PartialEq,
        Pool: StackPool<ItemType>, 
        Callback: Fn(&mut ItemType),
{
    let count = pool.count().clone();
    let mut i = &mut 0;

    while *i < count {
        if let Some(item) = pool.item_sorted(i) {
            handler(item);
        }
        *i += 1;
    } 
}

#[allow(unused)]
pub fn position<ItemType, Pool, Callback> (pool: &Pool, handler: Callback) -> Option<usize> 
where   ItemType: Copy + PartialEq,
        Pool: StackPool<ItemType>, 
        Callback: Fn(&ItemType) -> bool,
{
    let count = pool.count();
    let mut i = &mut 0;

    while *i < *count {
        if let Some(item) = pool.ref_sorted(i) {
            if handler(item) { return Some(*i); }
        }
        *i += 1;
    }
    return None
}

#[allow(unused)]
pub fn for_while<ItemType, Pool, Callback> (pool: &Pool, handler: Callback) -> Option<usize> 
where   ItemType: Copy + PartialEq,
        Pool: StackPool<ItemType>, 
        Callback: Fn(&ItemType) -> bool,
{
    let count = pool.count();
    let mut i = &mut 0;

    while *i < *count {
        if let Some(item) = pool.ref_sorted(i) {
            if !handler(item) { return Some(*i); }
        }
        *i += 1;
    }
    return None
}

#[allow(unused)]
pub fn push<ItemType, Pool> (pool: &mut Pool, item: ItemType)
where   ItemType: Copy + PartialEq,
        Pool: StackPool<ItemType>, 
{
    *pool.item_last() = Some(item);
    *pool.count_mut() += 1;
}

#[allow(unused)]
pub fn pop<ItemType, Pool> (pool: &mut Pool) -> Option<ItemType>
where   ItemType: Copy + PartialEq,
        Pool: StackPool<ItemType>, 
{
    if *pool.count() > 0 { 
        
        *pool.count_mut() -= 1;
        let item = pool.item_last().clone();
        *pool.item_last() = None;

        item
    } 
    else {
        None
    }
}

#[allow(unused)]
pub fn shift<ItemType, Pool> (pool: &mut Pool) -> Option<ItemType>
where   ItemType: Copy + PartialEq,
        Pool: StackPool<ItemType>, 
{
    match *pool.count() > 0 {
        true => remove_at(pool, 0),
        false => None,
    }
}

#[allow(unused)]
pub fn remove_at<ItemType, Pool> (pool: &mut Pool, position: usize) -> Option<ItemType>
where   ItemType: Copy + PartialEq,
        Pool: StackPool<ItemType>, 
{
    if *pool.count() > 0 {
        let item = pool.item_sorted(&position).clone();
        *pool.item_sorted(&position) = None;
        *pool.count_mut() -= 1;
        swap(pool, 0, *pool.count());

        item
    }
    else {
        None
    }
}

#[allow(unused)]
pub fn remove_target<ItemType, Pool> (pool: &mut Pool, item: &ItemType) -> Option<ItemType>
where   ItemType: Copy + PartialEq,
        Pool: StackPool<ItemType>, 
{
    find_position(pool, item).map(|i| remove_at(pool, i)).flatten()
}

#[allow(unused)]
pub fn swap<ItemType, Pool> (pool: &mut Pool, first_position: usize, second_position: usize)
where   ItemType: Copy + PartialEq,
        Pool: StackPool<ItemType>, 
{
    if first_position < *pool.count()
    && second_position < *pool.count()
    && first_position != second_position 
    {
        let first_order = pool.order_at(&first_position).clone();
        let second_order = pool.order_at(&second_position).clone();
        *pool.order_at(&first_position) = second_order;
        *pool.order_at(&second_position) = first_order;
    }
}

#[allow(unused)]
pub fn find_position<ItemType, Pool> (pool: &mut Pool, item: &ItemType) -> Option<usize>
where   ItemType: Copy + PartialEq,
        Pool: StackPool<ItemType>, 
{
    position(pool, |o| *o == *item)
}
