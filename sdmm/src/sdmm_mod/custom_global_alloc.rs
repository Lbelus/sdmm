

#[cfg(feature = "custom_allocator")]
use std::alloc::{GlobalAlloc, Layout};
#[cfg(feature = "custom_allocator")]
use std::cell::UnsafeCell;
#[cfg(feature = "custom_allocator")]
use std::ptr::null_mut;
#[cfg(feature = "custom_allocator")]
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

#[cfg(feature = "custom_allocator")]
const ARENA_SIZE: usize = 128 * 1024;
#[cfg(feature = "custom_allocator")]
const MAX_SUPPORTED_ALIGN: usize = 4096;

#[cfg(feature = "custom_allocator")]
#[repr(C, align(4096))] // 4096 == MAX_SUPPORTED_ALIGN
struct SimpleAllocator {
    arena: UnsafeCell<[u8; ARENA_SIZE]>,
    remaining: AtomicUsize, // we allocate from the top, counting down
}

#[cfg(feature = "custom_allocator")]
#[global_allocator]
static ALLOCATOR: SimpleAllocator = SimpleAllocator {
    arena: UnsafeCell::new([0x55; ARENA_SIZE]),
    remaining: AtomicUsize::new(ARENA_SIZE),
};

#[cfg(feature = "custom_allocator")]
unsafe impl Sync for SimpleAllocator {}

#[cfg(feature = "custom_allocator")]
unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
        // So we can safely use a mask to ensure alignment without worrying about UB.
        let align_mask_to_round_down = !(align - 1);

        if align > MAX_SUPPORTED_ALIGN {
            return null_mut();
        }

        let mut allocated = 0;
        if self
            .remaining
            .fetch_update(Relaxed, Relaxed, |mut remaining| {
                if size > remaining {
                    return None;
                }
                remaining -= size;
                remaining &= align_mask_to_round_down;
                allocated = remaining;
                Some(remaining)
            })
            .is_err()
        {
            return null_mut();
        };
        self.arena.get().cast::<u8>().add(allocated)
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[cfg(feature = "custom_allocator")]
pub fn basic_test_fn() -> bool {
    let _s = format!("allocating a string!");
    let currently = ALLOCATOR.remaining.load(Relaxed);
    println!("allocated so far: {}", ARENA_SIZE - currently);
    false
}