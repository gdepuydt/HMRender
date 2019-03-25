//! Feature flag related to Dynamically Sized Types.
//! 
//! For more information:
//! 
//! [Exotically size types](https://doc.rust-lang.org/nomicon/exotic-sizes.html)
//! 
//! [Coercions](https://doc.rust-lang.org/nomicon/coercions.html)
//! 
//! [Unsize](https://doc.rust-lang.org/unstable-book/library-features/unsize.html#unsize)
//! 
//! [coerce_unsized](https://doc.rust-lang.org/unstable-book/library-features/coerce-unsized.html?highlight=coerce#coerce_unsized)

#![feature(unsize,coerce_unsized)]

/// [Implementing an allocator](http://stevenlr.com/posts/handmade-rust-1-allocators/)
pub mod alloc;