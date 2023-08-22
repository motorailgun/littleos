use core::{alloc::GlobalAlloc, cell::UnsafeCell, ptr};
extern crate alloc;
use alloc::alloc::Layout;
use spin::Mutex;

pub struct DumbAllocator {
    head: Mutex<UnsafeCell<usize>>,
    tail: usize,
}

unsafe impl Sync for DumbAllocator {}

unsafe impl GlobalAlloc for DumbAllocator{
    // from embedded Rust book
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let head = self.head.lock().get();

        let align = layout.align();
        let res = *head % align;
        let start = if res == 0 { *head } else { *head + align - res };
        if start + align > self.tail {
            ptr::null_mut()
        } else {
            *head = start + align;
            start as *mut u8
        }
    }

    // this allocator does nothing on free
    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {}
}

impl DumbAllocator {
    pub const unsafe fn new_uninit() -> DumbAllocator {
        // DumbAllocator { head: Mutex::new(UnsafeCell::new(0)), tail: 0 }
        DumbAllocator { head: Mutex::new(UnsafeCell::new(0x900000)), tail: 0x1500000 }
    }

    pub unsafe fn init_instance(&mut self, start: usize, end: usize) {
        *(self.head.lock().get_mut()) = start;
        self.tail = end;
    }
}
