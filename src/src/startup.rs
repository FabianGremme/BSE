// insert other modules


/*
#![feature(lang_items)]
#![feature(ptr_internals)]
#![feature(const_mut_refs)]
#![allow(dead_code)]          // avoid warnings
#![allow(unused_variables)]   // avoid warnings
#![allow(unused_imports)]
#![allow(unused_macros)]
#![feature(restricted_std)]

extern crate spin; // we need a mutex in devices::cga_print
extern crate std; // standard lib
extern crate tinyrlibc; // ensure we have 'strlen', needed to build 'std'
extern crate rlibc; // ensure we have compiler-builtin-funcs, needed to build 'std'

extern crate alloc; // need for heap allocator


// insert other modules
#[macro_use]   // import macros, too
mod devices;
mod kernel;
mod user;
mod consts;


use devices::cga;         // shortcut for cga
use devices::cga_print;   // used to import code needed by println!
use devices::keyboard;    // shortcut for keyboard


use kernel::allocator;
use kernel::cpu;

use user::aufgabe1::text_demo;
use user::aufgabe1::keyboard_demo;
use user::aufgabe2::heap_demo;
use user::aufgabe2::sound_demo;



fn aufgabe1() {
   cga::clear();
   text_demo::run();
   //keyboard_demo::run();
}

fn aufgabe2() {
   heap_demo::run();
   //sound_demo::run();
}



#[no_mangle]
pub extern fn startup() {
*/
    // Speicherverwaltung initialisieren
   /* Hier muss Code eingefuegt werden */
/*
    aufgabe1();
    aufgabe2();

    loop{}
}


*/
#![no_std]
#![feature(lang_items)]
#![feature(ptr_internals)]
#![feature(const_mut_refs)]
#![allow(dead_code)]          // avoid warnings
#![allow(unused_variables)]   // avoid warnings
#![allow(unused_imports)]
#![allow(unused_macros)]

extern crate spin; // we need a mutex in devices::cga_print
extern crate tinyrlibc; // ensure we have 'strlen', needed to build 'std'
extern crate rlibc; // ensure we have compiler-builtin-funcs, needed to build 'std'

extern crate alloc; // need for heap allocator



// insert other modules
#[macro_use]   // import macros, too
mod devices;
mod kernel;
mod user;
mod consts;
use alloc::{boxed::Box};
use core::panic::PanicInfo;

use devices::cga;         // shortcut for cga
use devices::cga_print;   // used to import code needed by println!
use devices::keyboard;    // shortcut for keyboard
use devices::pit;


use kernel::allocator;
use kernel::cpu;
use kernel::threads::scheduler;
use kernel::threads::idle_thread;


//use user::aufgabe1::text_demo;
//use user::aufgabe1::keyboard_demo;
//use user::aufgabe2::heap_demo;
use user::aufgabe2::sound_demo;
use user::aufgabe3::keyboard_irq_demo;
use user::aufgabe4::corouts_demo;
use user::aufgabe4::hello_world_thread;
use user::aufgabe4::coop_thread_demo;
use user::aufgabe5::preempt_demo;
use user::aufgabe6::sync_preempt_demo;


mod mylib;
use crate::kernel::interrupts;

fn aufgabe5() {
    // Idle-Thread anlegen

    // Anwendung im Scheduler anmelden

    // Scheduler starten
}

#[no_mangle]
pub extern fn startup() {
	cga::clear();
    print!("do setup\n");
	// den allocator anschalten
    allocator::init();
    // Interrupt-Strukturen initialisieren
    interrupts::init();
    // Tastatur-Unterbrechungsroutine 'einstoepseln'
    /* Hier muss Code eingefuegt werden */
    keyboard::plugin();
    pit::plugin();

    // Interrupts an der CPU erlauben 
    /* Hier muss Code eingefuegt werden */
    cpu::enable_int();
    /*
    loop{
        keyboard_irq_demo::run();
    }*/
    //corouts_demo::run();
    //hello_world_thread::init();

    /*
    let idle = Box::new(idle_thread::IdleThread{});
    scheduler::Scheduler::ready(idle);
    coop_thread_demo::init();
    scheduler::Scheduler::schedule();*/

    //sound_demo::run();
    let idle = Box::new(idle_thread::IdleThread{currency:-1});
    scheduler::Scheduler::ready(idle);
    sync_preempt_demo::init();
    print!("setup done\n");
    scheduler::Scheduler::set_initialized(true);
    scheduler::Scheduler::schedule();
    loop{}

}


 #[panic_handler]
  fn panic(info: &PanicInfo) -> ! {
     println!("Panic: {}", info);
     loop {}
  }
