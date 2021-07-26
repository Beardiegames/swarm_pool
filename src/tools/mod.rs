//! Tools contains a set of Sized types that can be useful when creating pool objects.
//! Beacuse pool objects must Sized, all of their properties must be Sized.
//! 
//! To get a quick start without being immediatly bummed out by this, A few
//! tools are provided within this library. Although these tools appear to be equally
//! as awesome as the rest of this library, there are other libraries on crates.io
//! that provide more complete and optimized solutions on Sized objects.

pub mod byte_str;
pub mod sized_pool;
