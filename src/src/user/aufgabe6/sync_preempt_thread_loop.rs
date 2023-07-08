
use alloc::{boxed::Box};
use crate::devices::cga;         // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::kernel::cpu;
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;
use crate::devices::pcspk::delay;
use crate::mylib::spinlock;

use alloc::sync::Arc;


pub struct PreemptiveThreadLoop {
	cnt: u32,
    x: u32,
    y: u32,
    counter: Arc<spinlock::Spinlock<u32>>,
    currency:i32,
}

impl PreemptiveThreadLoop {

   pub fn new(x:u32, y:u32, counter:Arc<spinlock::Spinlock<u32>>,currency:i32)-> Box<PreemptiveThreadLoop> {
      Box::new(PreemptiveThreadLoop { cnt: 0, x:x, y:y, counter:counter,currency} )
   }
      
   pub fn get_raw_pointer (&mut self) -> *mut PreemptiveThreadLoop {
	   self
   }


}

impl thread::ThreadEntry for PreemptiveThreadLoop {
	
   fn run(&mut self, thread_object: *mut thread::Thread) {

        /* Hier muss Code eingefuegt werden */
       loop{
           cpu::disable_int();

           cga::setpos(self.x, self.y);
           if self.cnt != *self.counter.lock(){
               self.cnt =  *self.counter.lock();

           }
           cga::print_dec(self.cnt);
           self.cnt +=1;
           *self.counter.lock() +=1;
           cpu::enable_int();
           delay(10);
       }
   }

    fn get_currency(&mut self)->i32{
        return self.currency;
    }

    fn reward_punish(&mut self, value:i32){
        self.currency += value;
    }
}
