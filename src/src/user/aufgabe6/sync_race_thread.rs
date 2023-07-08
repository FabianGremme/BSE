
use alloc::{boxed::Box};
use crate::devices::cga;         // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::kernel::cpu;
use crate::kernel::threads::thread;
use crate::kernel::threads::scheduler;
use crate::devices::pcspk::delay;
use crate::mylib::spinlock;

use alloc::sync::Arc;


pub struct RaceThreadLoop {
    cnt: u32,
    x: u32,
    y: u32,
    is_winner: Arc<spinlock::Spinlock<u32>>,
    goal: u32,
    currency:i32,
}

impl RaceThreadLoop {

    pub fn new(x:u32, y:u32, is_winner:Arc<spinlock::Spinlock<u32>>, goal:u32, currency:i32)-> Box<RaceThreadLoop> {
        Box::new(RaceThreadLoop { cnt: 0, x, y, is_winner, goal,currency} )
    }

    pub fn get_raw_pointer (&mut self) -> *mut RaceThreadLoop {
        self
    }

}

impl thread::ThreadEntry for RaceThreadLoop {

    fn run(&mut self, thread_object: *mut thread::Thread) {

        /* Hier muss Code eingefuegt werden */
        loop{
            cpu::disable_int();

            cga::setpos(self.x, self.y);
            if self.cnt >= self.goal{
                if *self.is_winner.lock() !=1{
                    cga::print_str("Winner", cga::CGA_STD_ATTR);
                    *self.is_winner.lock() =1;
                    scheduler::Scheduler::exit(thread_object);

                }else{
                    cga::print_str("Looser", cga::CGA_STD_ATTR);
                    scheduler::Scheduler::exit(thread_object);
                }


            }else{
                cga::print_dec(self.cnt);
                self.cnt +=1;

            }
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
