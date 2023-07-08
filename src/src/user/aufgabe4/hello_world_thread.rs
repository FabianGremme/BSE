
use alloc::{boxed::Box};
use crate::devices::cga;  // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;
use crate::user::aufgabe4::coop_thread_loop;


pub struct HelloWorldThread {
    currency: i32,
}

impl HelloWorldThread {
  pub fn get_raw_pointer (&mut self) -> *mut HelloWorldThread {
	   self
   }

}

impl thread::ThreadEntry for HelloWorldThread {
    
    fn run(&mut self, thread_object: *mut thread::Thread) {
        println!("Hallo Welt von einem Thread!");
        scheduler::Scheduler::exit(thread_object);  
	}

    fn get_currency(&mut self) ->i32{
        return self.currency;
    }

    fn reward_punish(&mut self, value:i32){
        self.currency += value;
    }
}

pub fn init(currency: i32) {
    // Anwendung im Scheduler anmelden
    let hw_thread = Box::new(HelloWorldThread { currency} );
 	scheduler::Scheduler::ready(hw_thread);
}
