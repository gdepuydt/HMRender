use core::ffi::c_void;
use super::Layout;

/// More information about [the Option type](https://doc.rust-lang.org/core/option/enum.Option.html) 
/// More information about [unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)
/// More information about [c_void enum type](https://doc.rust-lang.org/core/ffi/enum.c_void.html)
pub trait Allocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Option<*mut c_void>;
    unsafe fn dealloc(&mut self, ptr: *mut c_void);
}