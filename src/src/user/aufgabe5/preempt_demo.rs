
use alloc::{boxed::Box};
use crate::devices::cga;           // shortcut for cga
use crate::devices::cga_print;     // used to import code needed by println! 
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;
use crate::user::aufgabe5::preempt_thread_loop;
use crate::user::aufgabe5::sound_thread;


pub struct PreemptiveThreadDemo {
    cnt: u32,
    currency:i32,
}

impl PreemptiveThreadDemo {

   pub fn new()-> Box<PreemptiveThreadDemo> {
      Box::new(PreemptiveThreadDemo { cnt: 0, currency:5} )
   }
      
   pub fn get_raw_pointer (&mut self) -> *mut PreemptiveThreadDemo {
	   self
   }
}

impl thread::ThreadEntry for PreemptiveThreadDemo {
	
    fn run(&mut self, thread_object: *mut thread::Thread) {
        scheduler::Scheduler::exit(thread_object);

    }

    fn get_currency(&mut self) ->i32{
        return self.currency;
    }

    fn reward_punish(&mut self, value:i32){
        self.currency += value;
    }

}


pub fn init() {
   // Anwendung im Scheduler anmelden
   let pd = Box::new(PreemptiveThreadDemo { cnt:0, currency: 5 } );
   scheduler::Scheduler::ready( pd );

    let sound = sound_thread::SoundThread::new(50);
    scheduler::Scheduler::ready( sound);

    let my_loop = preempt_thread_loop::PreemptiveThreadLoop::new(10, 15, 50);
    scheduler::Scheduler::ready(my_loop);
}
