
use alloc::{boxed::Box};
use crate::devices::cga;  // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;
use crate::user::aufgabe4::coop_thread_loop;
use crate::user::aufgabe4::hello_world_thread;
use crate::user::aufgabe4::coop_thread_loop::CoopThreadLoop;



pub struct CoopThreadDemo {
    cnt: u32,
    currency:i32,

}

impl CoopThreadDemo {
  pub fn get_raw_pointer (&mut self) -> *mut CoopThreadDemo {
	   self
   }
}

impl thread::ThreadEntry for CoopThreadDemo {
    
    fn run(&mut self, thread_object: *mut thread::Thread) {

       /* Hier muss Code eingefuegt werden */

       // Eine Loop stoppen
        loop {
            if self.cnt < 1000 {
                self.cnt += 1;
            } else {
                scheduler::Scheduler::kill(3);
                scheduler::Scheduler::exit(thread_object);

            }
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


pub fn init() {
   /* Hier muss Code eingefuegt werden */
   // Anwendung im Scheduler anmelden
    let thread_1 = coop_thread_loop::CoopThreadLoop::new(5, 5, 50);
    scheduler::Scheduler::ready(thread_1);

    let thread_2 = coop_thread_loop::CoopThreadLoop::new(25, 5, 50);
    scheduler::Scheduler::ready(thread_2);

    let thread_3 = coop_thread_loop::CoopThreadLoop::new(50, 5, 50);
    scheduler::Scheduler::ready(thread_3);

    scheduler::Scheduler::ready(Box::new(CoopThreadDemo{cnt: 0, currency:50}));
}
