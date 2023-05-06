use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

fn main() {
    let layout = Layout::from_size_align(1024, 8).unwrap();
    let shared_memory_ptr = unsafe { alloc(layout) };

    if shared_memory_ptr.is_null() {
        panic!("Failed to allocate shared memory.");
    }

    unsafe {
        ptr::write_bytes(shared_memory_ptr, 0, layout.size());
    }

    // Use shared_memory_ptr here for your systems programming tasks

    unsafe {
        dealloc(shared_memory_ptr, layout);
    }
}
