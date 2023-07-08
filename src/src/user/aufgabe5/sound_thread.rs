
use alloc::{boxed::Box};
use crate::devices::pcspk;   // shortcut for cga
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;


pub struct SoundThread {
    currency:i32,
}

impl SoundThread {

   pub fn new(currency:i32)-> Box<SoundThread> {
      Box::new(SoundThread { currency} )
   }
      
   pub fn get_raw_pointer (&mut self) -> *mut SoundThread {
	   self
   }
}

impl thread::ThreadEntry for SoundThread {
	
    fn run(&mut self, thread_object: *mut thread::Thread) {
           pcspk::tetris();
           scheduler::Scheduler::exit(thread_object);

	}

    fn get_currency(&mut self) ->i32{
        return self.currency;
    }

    fn reward_punish(&mut self, value:i32){
        self.currency += value;
    }
}
