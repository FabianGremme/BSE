
use alloc::{boxed::Box};
use crate::devices::cga;  // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;


pub struct CoopThreadLoop {
	cnt: u32,
    x: u32,
    y: u32,
    currency:i32,
}

impl CoopThreadLoop {

   pub fn new(x:u32, y:u32, currency:i32)-> Box<CoopThreadLoop> {
      Box::new(CoopThreadLoop { cnt: 0, x:x, y:y, currency:currency} )
   }
      
   pub fn get_raw_pointer (&mut self) -> *mut CoopThreadLoop {
	   self
   }
}

impl thread::ThreadEntry for CoopThreadLoop {
	
    fn run(&mut self, thread_object: *mut thread::Thread) {

      /* Hier muss Code eingefuegt werden */
        loop {
            // hier wird hochgezählt, geprintet und abgegeben
            // auch wird auf die richtige id gewartet, damit diese gestoppt wird
            cga::setpos(self.x, self.y);
            cga::print_dec(self.cnt);
            self.cnt +=1;
            scheduler::Scheduler::yield_cpu(thread_object);
        }

	}
    fn get_currency(&mut self) ->i32{
        return self.currency;
    }

    fn reward_punish(&mut self, value:i32){
        self.currency += value;
    }
}
