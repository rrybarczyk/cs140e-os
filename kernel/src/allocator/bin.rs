use std::fmt;
use alloc::heap::{AllocErr, Layout};
use std::cmp::{min, max};

use allocator::util::*;
use allocator::linked_list::LinkedList;
//use super::super::console::kprintln;

const BINS_SIZE : usize = 32;
const MIN_SLAB_SIZE_BITS : usize = 3;

/// A simple allocator that allocates based on size classes.
//#[derive(Debug)]
pub struct Allocator {
    bins : [LinkedList; BINS_SIZE],
}

impl Allocator {
    /// Creates a new bin allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {
        let mut bins = [LinkedList::new(); BINS_SIZE];
        let mut start = start;

        while start < end {
            let sz = min(1 << start.trailing_zeros(),
                        (end - start).next_power_of_two() << 1);
            if sz >= 1 << MIN_SLAB_SIZE_BITS {
                unsafe {
                    bins[sz.trailing_zeros() as usize]
                        .push(start as *mut usize);
                }
            }
            start += sz;
        }

        Allocator{
            bins : bins
        }
    }

    /// Allocates memory. Returns a pointer meeting the size and alignment
    /// properties of `layout.size()` and `layout.align()`.
    ///
    /// If this method returns an `Ok(addr)`, `addr` will be non-null address
    /// pointing to a block of storage suitable for holding an instance of
    /// `layout`. In particular, the block will be at least `layout.size()`
    /// bytes large and will be aligned to `layout.align()`. The returned block
    /// of storage may or may not have its contents initialized or zeroed.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure that `layout.size() > 0` and that
    /// `layout.align()` is a power of two. Parameters not meeting these
    /// conditions may result in undefined behavior.
    ///
    /// # Errors
    ///
    /// Returning `Err` indicates that either memory is exhausted
    /// (`AllocError::Exhausted`) or `layout` does not meet this allocator's
    /// size or alignment constraints (`AllocError::Unsupported`).
    pub fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
//        kprintln!("allocating object with size {}, align {}", layout.size(), layout.align());
        if !layout.align().is_power_of_two() {
            return Err(AllocErr::Unsupported {
                details:"alignment is not power of 2"
            });
        } else if layout.size() <= 0 {
            return Err(AllocErr::Unsupported {
                details:"size is 0"
            });
        }

        let size = align_up(layout.size().next_power_of_two(),
                            max(layout.align(), 1 << MIN_SLAB_SIZE_BITS));
        let index = size.trailing_zeros() as usize;

        for i in index..self.bins.len() {
            if self.bins[i].is_empty() { continue }
            let addr = self.bins[i].pop().unwrap() as *mut u8;
            for j in index..i {
                unsafe {
                    let buddy_addr = addr.add(1 << j) as *mut usize;
                    self.bins[j].push(buddy_addr);
                }
            }
            return Ok(addr);
        }
        Err(AllocErr::Exhausted { request: layout })
    }

    /// Deallocates the memory referenced by `ptr`.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure the following:
    ///
    ///   * `ptr` must denote a block of memory currently allocated via this
    ///     allocator
    ///   * `layout` must properly represent the original layout used in the
    ///     allocation call that returned `ptr`
    ///
    /// Parameters not meeting these conditions may result in undefined
    /// behavior.
    pub fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
//        kprintln!("dealloc ptr {:?}, sz {}, align {}", ptr, layout.size(), layout.align());
        let size = align_up(layout.size().next_power_of_two(),
                            max(layout.align(), 1 << MIN_SLAB_SIZE_BITS)) as usize;
        let index = size.trailing_zeros() as usize;
        let mut my_addr = ptr as usize;
        for i in index..self.bins.len() {
            let buddy_addr = my_addr ^ (1 << i);
            let mut found_buddy = false;
            for node in self.bins[i].iter_mut() {
                if node.value() as usize == buddy_addr {
                    node.pop();
                    my_addr = min(buddy_addr, my_addr);
                    found_buddy = true;
                    break;
                }
            }

            if !found_buddy {
                unsafe {self.bins[i].push(my_addr as *mut usize);}
                break;
            }
        }
    }
}

impl fmt::Debug for Allocator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Bin Allocator")
         .field("bins", &self.bins)
         .finish()
    }
}
