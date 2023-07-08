
use alloc::{boxed::Box};
use crate::devices::cga as cga;  // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::kernel::threads::thread as thread;
use crate::kernel::threads::scheduler as scheduler;


pub struct IdleThread {
	pub currency:i32,
}

impl thread::ThreadEntry for IdleThread {
	
    fn run(&mut self, thread_object: *mut thread::Thread) {
       	let id = thread::Thread::get_tid(thread_object);
		scheduler::Scheduler::set_initialized(true);

		//cga::print_str("idle_thread::run", cga::CGA_STD_ATTR);
		loop {
		   // println!("idle: tid={}", id);
           scheduler::Scheduler::yield_cpu(thread_object);
		}
	}

	fn get_currency(&mut self)->i32{
		return -1;
	}

	fn reward_punish(&mut self, value:i32){

	}


	
}





