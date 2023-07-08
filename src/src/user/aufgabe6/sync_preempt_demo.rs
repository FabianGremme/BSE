
use alloc::{boxed::Box};
use crate::devices::cga;           // shortcut for cga
use crate::devices::cga_print;     // used to import code needed by println! 
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;
use crate::user::aufgabe6::sync_preempt_thread_loop;
use crate::user::aufgabe6::sound_thread;
use crate::user::aufgabe6::sync_race_thread::RaceThreadLoop;
use crate::mylib::spinlock;

use alloc::sync::Arc;


pub struct PreemptiveThreadDemo {
    cnt: u32,
    currency:i32,
}

impl PreemptiveThreadDemo {

   pub fn new(currency:i32)-> Box<PreemptiveThreadDemo> {
      Box::new(PreemptiveThreadDemo { cnt: 0, currency} )
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
   let counter = Arc::new(spinlock::Spinlock::new(0));
    let finish = Arc::new(spinlock::Spinlock::new(0));
    let goal = 1000;

   let pd = Box::new(PreemptiveThreadDemo { cnt:0, currency:50} );
   scheduler::Scheduler::ready( pd );

    let sound = sound_thread::SoundThread::new(20000);
    scheduler::Scheduler::ready( sound);

    let my_loop1 = sync_preempt_thread_loop::PreemptiveThreadLoop::new(10, 10, counter.clone(), 100);
    scheduler::Scheduler::ready(my_loop1);

    let my_loop2 = sync_preempt_thread_loop::PreemptiveThreadLoop::new(50, 10, counter.clone(), 1000);
    scheduler::Scheduler::ready(my_loop2);

    let race_loop1 = RaceThreadLoop::new(10, 20, finish.clone(), goal, 500);
    scheduler::Scheduler::ready(race_loop1);

    let race_loop2 = RaceThreadLoop::new(50, 20, finish.clone(), goal, 1500);
    scheduler::Scheduler::ready(race_loop2);
}
