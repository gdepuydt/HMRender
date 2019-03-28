//! Example allocator implemented using Windows' HeapAlloc and HeapFree

use core::ffi::c_void;
//https://doc.rust-lang.org/core/ptr/struct.NonNull.html
use core::ptr::NonNull;
use super::Layout;
use super::Allocator;


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
    fn HeapAlloc(heap: HANDLE, flags: u32, byte: usize) -> *mut c_void; // should be equivalent to HANDLE, no?
    fn HeapFree(heap: HANDLE, flags: u32, mem: *mut c_void) -> BOOL;
}
///implement Default trait for giving a type a useful default value
impl Default for Win32HeapAllocator {
    fn default() -> Self {
        Self {
            heap: unsafe {GetProcessHeap()},
        }
    }
}

impl Allocator for &Win32HeapAllocator {
    unsafe fn alloc (&mut self, layout: Layout) -> Option<NonNull<c_void>> {
        let ptr = HeapAlloc(self.heap, 0, layout.size);
        if ptr.is_null() {
            None
        }
        else {
            // Creates a new NonNull pointer type.
            // More information can be found at 
            // https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.new_unchecked
            Some(NonNull::new_unchecked(ptr))
        }

    }

    unsafe fn dealloc (&mut self, ptr: *mut c_void) {
        HeapFree(self.heap, 0, ptr);
    }




}