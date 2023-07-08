/*****************************************************************************
 *                                                                           *
 *                                 B U M P                                   *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Eine sehr einfache Heap-Verwaltung, welche freigegebenen *
 *                  Speicher nicht mehr nutzen kann.                         *
 *                                                                           *
 * Autor:           Philipp Oppermann                                        *
 *                  https://os.phil-opp.com/allocator-designs/               *
 *                  Modified by Michael Schoettner, 15.3.2022                *
 *****************************************************************************/

use super::{align_up, Locked};
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;


pub struct BumpAllocator {

   /* Hier muss Code eingefuegt werden */
   start:usize,
   end:usize,
    next:usize
   
}

impl BumpAllocator {
    // Creates a new empty bump allocator.
    pub const fn new() -> Self {

       /* Hier muss Code eingefuegt werden */
        return BumpAllocator{ start: 0, end: 0, next: 0 };

    }

    /*
     * Initializes the bump allocator with the given heap bounds.
     * 
     * This method is unsafe because the caller must ensure that the given
     *  memory range is unused. Also, this method must be called only once.
     */
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {

       /* Hier muss Code eingefuegt werden */
        self.start = heap_start;
        self.next = heap_start;
        self.end = heap_start + heap_size;
    }

    // Dump free list
    pub fn dump_free_list(&mut self) {

       /* Hier muss Code eingefuegt werden */
        // hier soll nur alles weggeworfen werden, da keine Liste ex
        self.next = self.start;
 		
	}

   pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
       let ptr = self.next as u64;
       let mut alignment = 0;
       if ptr % 8 != 0{
           alignment = ptr % 8;
       }

       /* Hier muss Code eingefuegt werden */
       self.next = self.next + layout.size() + alignment as usize;
       return (ptr + alignment) as *mut u8;

   }
   
   pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
      //println!("   dealloc: size={}, align={}; not supported", layout.size(), layout.align());
   }

    pub fn get_start(&mut self) -> usize{
        return self.start;
    }

    pub fn get_next(&mut self) -> usize{
        return self.next;
    }

    pub fn get_end(&mut self) -> usize{
        return self.end;
    }

}

// Trait required by the Rust runtime for heap allocations
unsafe impl GlobalAlloc for Locked<BumpAllocator> {
	
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.lock().dealloc(ptr, layout);
    }

}

