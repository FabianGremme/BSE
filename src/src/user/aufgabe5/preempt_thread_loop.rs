
use alloc::{boxed::Box};
use crate::devices::cga;         // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::kernel::cpu;
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;


pub struct PreemptiveThreadLoop {
	cnt: u32,
    x: u32,
    y: u32,
    currency:i32,
}

impl PreemptiveThreadLoop {

   pub fn new(x:u32, y:u32, currency:i32)-> Box<PreemptiveThreadLoop> {
      Box::new(PreemptiveThreadLoop { cnt: 0, x, y, currency} )
   }
      
   pub fn get_raw_pointer (&mut self) -> *mut PreemptiveThreadLoop {
	   self
   }
}

impl thread::ThreadEntry for PreemptiveThreadLoop {
	
   fn run(&mut self, thread_object: *mut thread::Thread) {

        /* Hier muss Code eingefuegt werden */
       loop{
           cga::setpos(self.x, self.y);
           cga::print_dec(self.cnt);
           self.cnt +=1;
       }
   }

    fn get_currency(&mut self) ->i32{
        return self.currency;
    }

    fn reward_punish(&mut self, value:i32){
        self.currency += value;
    }
}
