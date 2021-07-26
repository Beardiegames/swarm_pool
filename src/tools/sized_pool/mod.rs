//! A set of Sized pools that can be use kindof like a sizable Array or Vec.
//! 
//! These pools come in 7 different capacity flavours: 16, 32, 64, 128, 256, 512 and 1024.
//! These pools cannot exceed their maximum capacity. 

mod pool16; 
mod pool32;
mod pool64;
mod pool128;
mod pool256;
mod pool512;
mod pool1024;

pub use pool16::SizedPool16;
pub use pool32::SizedPool32;
pub use pool64::SizedPool64;
pub use pool128::SizedPool128;
pub use pool256::SizedPool256;
pub use pool512::SizedPool512;
pub use pool1024::SizedPool1024;

/// The StackPool trait is used by this module to communicate with the different pool
/// types on a polymorphic level. 
/// 
/// It was not meant to be public, but hiding the ugly 
/// beast would dissapoint the compiler when used externally.
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
/// Loop through all pushed items and edit them using a callback handler.
/// 
/// # Example
/// ```
/// use swarm::tools::sized_pool::SizedPool16;
/// use swarm::tools::sized_pool;
/// 
/// fn main() {
///     let mut pool: SizedPool16<u8> = SizedPool16::new();
///     sized_pool::push(&mut pool, 1);
///     sized_pool::push(&mut pool, 2);
///     sized_pool::push(&mut pool, 3);
///     
///     sized_pool::for_each(&mut pool, |item| *item *= 2 );
/// 
///     assert_eq!(sized_pool::get_ref(&pool, 0), &Some(2));
///     assert_eq!(sized_pool::get_ref(&pool, 1), &Some(4));
///     assert_eq!(sized_pool::get_ref(&pool, 2), &Some(6));
/// }
/// ```
pub fn for_each<ItemType, Pool, Callback>(pool: &mut Pool, handler: Callback) 
where   
    ItemType: Copy + PartialEq,
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
/// Find the position of an item within the pool. Returns Some(usize) 
/// if the callback handler returns true or None if no callbacks returned a success
/// 
/// # Example
/// ```
/// use swarm::tools::sized_pool::SizedPool16;
/// use swarm::tools::sized_pool;
/// use swarm::tools::byte_str::ByteStr;
/// 
/// fn main() {
///     let mut pool: SizedPool16<u8> = SizedPool16::new();
///     sized_pool::push(&mut pool, "A".as_bytes()[0]); // index position 0
///     sized_pool::push(&mut pool, "B".as_bytes()[0]); // index position 1
///     sized_pool::push(&mut pool, "C".as_bytes()[0]); // index position 2
///     
///     let position = sized_pool::position(&mut pool, |item| *item == "B".as_bytes()[0] );
/// 
///     assert_eq!(&position, &Some(1)); // the position of "B" is 1
///     assert_eq!(sized_pool::get_ref(&pool, position.unwrap()).unwrap(), "B".as_bytes()[0]);
/// }
/// ```
pub fn position<ItemType, Pool, Callback> (pool: &Pool, handler: Callback) -> Option<usize> 
where   
    ItemType: Copy + PartialEq,
    Pool: StackPool<ItemType>, 
    Callback: Fn(&ItemType) -> bool,
{
    let count = pool.count();
    let mut i = &mut 0;

    while *i < *count {
        if let Some(item) = pool.ref_sorted(i) {
            if handler(item) { return Some(i.clone()); }
        }
        *i += 1;
    }
    return None
}


#[allow(unused)]
/// Find the position of an item within the pool. Returns Some(usize) 
/// if the item was found, else it returns None.
/// 
/// # Example
/// ```
/// use swarm::tools::sized_pool::SizedPool16;
/// use swarm::tools::sized_pool;
/// use swarm::tools::byte_str::ByteStr;
/// 
/// fn main() {
///     let mut pool: SizedPool16<u8> = SizedPool16::new();
///     sized_pool::push(&mut pool, "A".as_bytes()[0]); // index position 0
///     sized_pool::push(&mut pool, "B".as_bytes()[0]); // index position 1
///     sized_pool::push(&mut pool, "C".as_bytes()[0]); // index position 2
///     
///     let position = sized_pool::find(&mut pool, &"B".as_bytes()[0]);
/// 
///     assert_eq!(&position, &Some(1)); // the position of "B" is 1
///     assert_eq!(sized_pool::get_ref(&pool, position.unwrap()).unwrap(), "B".as_bytes()[0]);
/// }
/// ```
pub fn find<ItemType, Pool> (pool: &mut Pool, item: &ItemType) -> Option<usize>
where   ItemType: Copy + PartialEq,
        Pool: StackPool<ItemType>, 
{
    position(pool, |o| *o == *item)
}


#[allow(unused)]
/// Loop through all items and keep looping until the callback handler returns false.
/// Returns the position where the loop was interrupted or None if all 
/// callback handlers returned true.  
/// 
/// # Example
/// ```
/// use swarm::tools::sized_pool::SizedPool16;
/// use swarm::tools::sized_pool;
/// 
/// fn main() {
///     let mut pool: SizedPool16<u8> = SizedPool16::new();
///     sized_pool::push(&mut pool, 10); // index position 0
///     sized_pool::push(&mut pool, 20); // index position 1
///     sized_pool::push(&mut pool, 30); // index position 2
///     
///     let position = sized_pool::for_while(&mut pool, |item| *item < 25);
/// 
///     assert_eq!(&position, &Some(2)); // at index position '2' the loop was interrupted
/// }
/// ```
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
/// Add a new item to the pool.
/// 
/// # Example
/// ```
/// use swarm::tools::sized_pool::SizedPool16;
/// use swarm::tools::sized_pool;
/// 
/// fn main() {
///     let mut pool: SizedPool16<u8> = SizedPool16::new();
///     sized_pool::push(&mut pool, 10); // index position 0
///     sized_pool::push(&mut pool, 20); // index position 1
///      
///     assert_eq!(sized_pool::get_ref(&pool, 0), &Some(10));
///     assert_eq!(sized_pool::get_ref(&pool, 1), &Some(20));
/// }
/// ```
pub fn push<ItemType, Pool> (pool: &mut Pool, item: ItemType)
where   ItemType: Copy + PartialEq,
        Pool: StackPool<ItemType>, 
{
    *pool.item_last() = Some(item);
    *pool.count_mut() += 1;
}

#[allow(unused)]
/// Removes the last item from the pool and returns that item.
/// 
/// # Example
/// ```
/// use swarm::tools::sized_pool::SizedPool16;
/// use swarm::tools::sized_pool;
/// 
/// fn main() {
///     let mut pool: SizedPool16<u8> = SizedPool16::new();
///     sized_pool::push(&mut pool, 10); // index position 0
///     sized_pool::push(&mut pool, 20); // index position 1
///     assert_eq!(sized_pool::count(&pool), &2);
/// 
///     let popped_item = sized_pool::pop(&mut pool);
///      
///     assert_eq!(sized_pool::count(&pool), &1);
///     assert_eq!(popped_item, Some(20));
/// }
/// ```
pub fn pop<ItemType, Pool> (pool: &mut Pool) -> Option<ItemType>
where   
    ItemType: Copy + PartialEq,
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
/// Removes the first item from the pool and returns that item.
/// 
/// # Example
/// ```
/// use swarm::tools::sized_pool::SizedPool16;
/// use swarm::tools::sized_pool;
/// 
/// fn main() {
///     let mut pool: SizedPool16<u8> = SizedPool16::new();
///     sized_pool::push(&mut pool, 10); // index position 0
///     sized_pool::push(&mut pool, 20); // index position 1
///     assert_eq!(sized_pool::count(&pool), &2);
/// 
///     let shifted_item = sized_pool::shift(&mut pool);
///      
///     assert_eq!(sized_pool::count(&pool), &1);
///     assert_eq!(shifted_item, Some(10));
/// }
/// ```
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
/// Returns the number of items in the pool.
/// 
/// # Example
/// ```
/// use swarm::tools::sized_pool::SizedPool16;
/// use swarm::tools::sized_pool;
/// 
/// fn main() {
///     let mut pool: SizedPool16<u8> = SizedPool16::new();
///     assert_eq!(sized_pool::count(&pool), &0);
/// 
///     sized_pool::push(&mut pool, 10); // index position 0
///     assert_eq!(sized_pool::count(&pool), &1);
/// 
///     sized_pool::push(&mut pool, 20); // index position 1
///     assert_eq!(sized_pool::count(&pool), &2);
/// }
/// ```
pub fn count<ItemType, Pool>(pool: &Pool) -> &usize 
where   
    ItemType: Copy + PartialEq,
    Pool: StackPool<ItemType>, 
{
    &pool.count()
}

#[allow(unused)]
/// Remove an item at position. 
/// Returns the removed item or None if it wans't found. 
/// 
/// # Example
/// ```
/// use swarm::tools::sized_pool::SizedPool16;
/// use swarm::tools::sized_pool;
/// 
/// fn main() {
///     let mut pool: SizedPool16<u8> = SizedPool16::new();
///     sized_pool::push(&mut pool, 10); // index position 0
///     sized_pool::push(&mut pool, 20); // index position 1
///     sized_pool::push(&mut pool, 30); // index position 2
/// 
///     let position = sized_pool::position(&pool, |item| *item == 20);
///     assert_eq!(position, Some(1), "expected position to be Some(1)");
/// 
///     let removed = sized_pool::remove_at(&mut pool, position.unwrap());
///     assert_eq!(removed, Some(20), "expected removed to be Some(20)");
/// 
///     assert_eq!(sized_pool::count(&pool), &2, "expected pool count to be 2");
///     assert_eq!(sized_pool::get_ref(&pool, 2), &None, "expected item 2 to be None");
///     assert_eq!(sized_pool::get_ref(&pool, 1), &Some(30), "expected item 1 to be Some(30)"); // second position should point to the last item
///     
///     let find_20 = sized_pool::position(&pool, |item| *item == 20);
///     assert_eq!(find_20, None);
/// }
/// ```
pub fn remove_at<ItemType, Pool> (pool: &mut Pool, position: usize) -> Option<ItemType>
where   ItemType: Copy + PartialEq,
        Pool: StackPool<ItemType>, 
{
    if *pool.count() > 0 {
        let item = pool.item_sorted(&position).clone();
        *pool.item_sorted(&position) = None;
        swap(pool, &position.clone(), &(pool.count()-1));
        *pool.count_mut() -= 1;

        item
    }
    else {
        None
    }
}

#[allow(unused)]
/// Remove an item that equals another item. 
/// Returns the removed item or None if it wans't found. 
/// 
/// # Example
/// ```
/// use swarm::tools::sized_pool::SizedPool16;
/// use swarm::tools::sized_pool;
/// 
/// fn main() {
///     let mut pool: SizedPool16<u8> = SizedPool16::new();
///     sized_pool::push(&mut pool, 10); // index position 0
///     sized_pool::push(&mut pool, 20); // index position 1
///     sized_pool::push(&mut pool, 30); // index position 2
/// 
///     let position = sized_pool::position(&pool, |item| *item == 20);
///     let removed = sized_pool::remove_equal(&mut pool, 20);
///      
///     assert_eq!(removed, Some(20));
///     assert_eq!(sized_pool::count(&pool), &2);
///     assert_eq!(sized_pool::get_ref(&pool, 1), &Some(30)); // second position should point to the last item
///     
///     let find_20 = sized_pool::position(&pool, |item| *item == 20);
///     assert_eq!(find_20, None);
/// }
/// ```
pub fn remove_equal<ItemType, Pool> (pool: &mut Pool, item: ItemType) -> Option<ItemType>
where   ItemType: Copy + PartialEq,
        Pool: StackPool<ItemType>, 
{
    find(pool, &item).map(|i| remove_at(pool, i)).flatten()
}

#[allow(unused)]
/// Returns a mutable reference of an item from the pool at position.
/// 
/// # Example
/// ```
/// use swarm::tools::sized_pool::SizedPool16;
/// use swarm::tools::sized_pool;
/// 
/// fn main() {
///     let mut pool: SizedPool16<u8> = SizedPool16::new();
///     sized_pool::push(&mut pool, 10); // index position 0
///     sized_pool::push(&mut pool, 20); // index position 1
/// 
///     let get_item_0 = sized_pool::get_item(&mut pool, 0);
///     assert_eq!(get_item_0, &mut Some(10));
/// 
///     let get_item_1 = sized_pool::get_ref(&pool, 1);
///     assert_eq!(get_item_1, &Some(20));
/// }
/// ```
pub fn get_item<ItemType, Pool>(pool: &mut Pool, position: usize) -> &mut Option<ItemType> 
where   
    ItemType: Copy + PartialEq,
    Pool: StackPool<ItemType>, 
{
    pool.item_sorted(&position)
}

#[allow(unused)]
/// Returns a immutable reference of an item from the pool at position.
/// 
/// # Example
/// ```
/// use swarm::tools::sized_pool::SizedPool16;
/// use swarm::tools::sized_pool;
/// 
/// fn main() {
///     let mut pool: SizedPool16<u8> = SizedPool16::new();
///     sized_pool::push(&mut pool, 10); // index position 0
///     sized_pool::push(&mut pool, 20); // index position 1
/// 
///     let get_item_0 = sized_pool::get_item(&mut pool, 0);
///     assert_eq!(get_item_0, &mut Some(10));
/// 
///     let get_item_1 = sized_pool::get_ref(&pool, 1);
///     assert_eq!(get_item_1, &Some(20));
/// }
/// ```
pub fn get_ref<ItemType, Pool>(pool: &Pool, position: usize) -> &Option<ItemType> 
where   
    ItemType: Copy + PartialEq,
    Pool: StackPool<ItemType>, 
{
    pool.ref_sorted(&position)
}

#[allow(unused)]
/// Swap the positions of two items in the pool.
/// 
/// # Example
/// ```
/// use swarm::tools::sized_pool::SizedPool16;
/// use swarm::tools::sized_pool;
/// 
/// fn main() {
///     let mut pool: SizedPool16<u8> = SizedPool16::new();
///     sized_pool::push(&mut pool, 10); // index position 0
///     sized_pool::push(&mut pool, 20); // index position 1
///     sized_pool::push(&mut pool, 30); // index position 2
/// 
///     assert_eq!(sized_pool::get_ref(&pool, 0), &Some(10));
///     assert_eq!(sized_pool::get_ref(&pool, 1), &Some(20));
///     assert_eq!(sized_pool::get_ref(&pool, 2), &Some(30));
/// 
///     sized_pool::swap(&mut pool, &1, &2);
///     
///     assert_eq!(sized_pool::get_ref(&pool, 0), &Some(10));
///     assert_eq!(sized_pool::get_ref(&pool, 1), &Some(30));
///     assert_eq!(sized_pool::get_ref(&pool, 2), &Some(20));
/// 
///     sized_pool::swap(&mut pool, &0, &2);
///     
///     assert_eq!(sized_pool::get_ref(&pool, 0), &Some(20));
///     assert_eq!(sized_pool::get_ref(&pool, 1), &Some(30));
///     assert_eq!(sized_pool::get_ref(&pool, 2), &Some(10));
/// 
///     sized_pool::swap(&mut pool, &0, &1);
///     
///     assert_eq!(sized_pool::get_ref(&pool, 0), &Some(30));
///     assert_eq!(sized_pool::get_ref(&pool, 1), &Some(20));
///     assert_eq!(sized_pool::get_ref(&pool, 2), &Some(10));
/// }
/// ```
pub fn swap<ItemType, Pool> (pool: &mut Pool, first_position: &usize, second_position: &usize)
where   ItemType: Copy + PartialEq,
        Pool: StackPool<ItemType>, 
{
    // if first_position < *pool.count()
    // && second_position < *pool.count()
    // && first_position != second_position 
    // {
        let first_order = pool.order_at(first_position).clone();
        let second_order = pool.order_at(second_position).clone();
        *pool.order_at(first_position) = second_order;
        *pool.order_at(second_position) = first_order;
    //}
}
