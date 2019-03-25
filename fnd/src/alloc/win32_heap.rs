//! Example allocator implemented using Windows' HeapAlloc and HeapFree

use core::ffi::c_void;
//https://doc.rust-lang.org/core/ptr/struct.NonNull.html
use core::ptr::NonNull;
use super::Layout;
//use super::Allocator;


type HANDLE = *mut c_void;
type BOOL = i32;

pub struct Win32HeapAllocator {
    heap: HANDLE,
}

// The #[link(...)] attribute is used to instruct the linker 
// to link against the kernel32 library so the symbols are resolved.
#[link(name = "kernel32")]
// Set the calling convention.
// For more information on the system ABI can be found 
// [here] (https://doc.rust-lang.org/nomicon/ffi.html#foreign-calling-conventions)
extern "system" {
    // For more details about these win32 functions can be found,
    // here: https://docs.microsoft.com/en-us/windows/desktop/api/heapapi/
    fn GetProcessHeap() -> HANDLE;
    // TODO Geert: add HeapAlloc and HeapFree functions
}