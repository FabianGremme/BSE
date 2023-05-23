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

//use user::aufgabe1::text_demo;
//use user::aufgabe1::keyboard_demo;
//use user::aufgabe2::heap_demo;
//use user::aufgabe2::sound_demo;
use user::aufgabe3::keyboard_irq_demo;

mod mylib;
use crate::kernel::interrupts;

#[no_mangle]
pub extern fn startup() {
	cga::clear();
	// den allocator anschalten
    allocator::init();
    // Interrupt-Strukturen initialisieren
    interrupts::init();
    // Tastatur-Unterbrechungsroutine 'einstoepseln'
    /* Hier muss Code eingefuegt werden */
    keyboard::plugin();
    print!("setup done\n");

    // Interrupts an der CPU erlauben 
    /* Hier muss Code eingefuegt werden */
    cpu::enable_int();
    loop{
        keyboard_irq_demo::run();

    }
}

