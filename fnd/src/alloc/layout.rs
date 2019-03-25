use core::mem::{size_of, align_of};

///Layout defines the size and alignment of the memory we want to allocate.
pub struct Layout {
    pub size: usize,
    pub align: usize,
}

impl Layout {
    /// Create a new Layout with a given size in bytes and an hardcoded alignment of 4 bytes
    pub fn new(size: usize) -> Self {
        Self {
            size,
            align: 4,
        }
    }
    /// Create a Layout from an existing type
    pub fn from_type<T>() -> Self {
        Self {
            /// Size in bytes of the type
            size: size_of::<T>(),
            /// Default alignment bytes of the type
            align: align_of::<T>(),
        }
    }
}


