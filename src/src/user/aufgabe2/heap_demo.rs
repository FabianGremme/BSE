/*
use crate::devices::cga as cga;  
use crate::devices::cga_print;       
use crate::devices::key as key;     
use crate::devices::keyboard as keyboard;  
use crate::kernel::allocator as allocator;  
use alloc::{boxed::Box, vec::Vec};



// Hilfsfunktion: Auf Return-Taste warten
fn wait_for_return() {
	
	println!("");
	println!("");
    println!("Weiter mit <ENTER>");

   loop {
      let mut key: key::Key = keyboard::key_hit();
        
      if key.valid() == true {
		  if key.get_ascii() == 13 { break; }
      }
   }
}


fn demo() {
*/
    /* Hier muss Code eingefuegt werden */
     //für bump
    /*// free heap allocated struct before return
    cga::clear();
    println!("Dieser Test soll mit dem Bump- Allocator ausgefuehrt werden:");
    println!("HEAP_START liegt bei {} und HEAP_SIZE ist {}", allocator::HEAP_START, allocator::HEAP_SIZE);

    wait_for_return();
    cga::clear();

    allocator::init();
    println!("Nun soll eine Box alloziiert werden: ");
    let b1 = Box::new(5);
    println!("b1: Groesse {}", b1);
    allocator::print_status();
    let b2 = Box::new(25);
    println!("b2: Groesse {}", b2);
    allocator::print_status();
    let b2 = Box::new(999999999);
    allocator::print_status();
    wait_for_return();
    cga::clear();*/
    
/*
    
    // nun fuer die Liste:
    allocator::init();
    cga::clear();
    println!("Dieser Test soll mit dem List - Allocator ausgefuehrt werden:");
    println!("HEAP_START liegt bei {} und HEAP_SIZE ist {}", allocator::HEAP_START, allocator::HEAP_SIZE);

    wait_for_return();
    cga::clear();

    println!("Nun soll eine Box alloziiert werden: ");
    let b1 = Box::new(500);
    println!("b1: Groesse {}", b1);
    allocator::dump_free_list();

    let b2 = Box::new(700);
    println!("b2: Groesse {}", b2);
    allocator::dump_free_list();

    wait_for_return();
    cga::clear();

    drop(b2);
    drop(b1);
    allocator::dump_free_list();


    println!("*** STOP ****");
    loop {}
  
    
    



}



pub fn run () {

    demo();

    // Hier muss Code eingefuegt werden

}
*/