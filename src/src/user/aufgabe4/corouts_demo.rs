
use alloc::{boxed::Box};
use crate::devices::cga;         // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::kernel::corouts::coroutine;


struct Loop {
	cnt: u32,
	x: u32,
	y:u32
}


impl coroutine::CoroutineEntry for Loop {
    fn run(&mut self, object: *mut coroutine::Coroutine) {

       /* Hier muss Code eingefuegt werden */
		loop{
			cga::setpos(self.x, self.y);
			cga::print_dec(self.cnt);
			self.cnt +=1;
			coroutine::Coroutine::switch_to_next(object);
		}
	}
}

pub fn run() {

   /* Hier muss Code eingefuegt werden
    * 
    * Die 3 Koroutinen einrichten, verketten und die 1. starten
    *
    */


	let mut corout1 = coroutine::Coroutine::new(1, Box::new(Loop { cnt: 0, y:5, x:5 }));

	let mut corout2 = coroutine::Coroutine::new(2, Box::new(Loop { cnt: 0, y:5, x:20 }));

	let mut corout3 = coroutine::Coroutine::new(3, Box::new(Loop { cnt: 0, y:5, x:50 }));


	corout1.as_mut().set_next(corout2.as_mut().get_raw_pointer());
	corout2.as_mut().set_next(corout3.as_mut().get_raw_pointer());
	corout3.as_mut().set_next(corout1.as_mut().get_raw_pointer());


	//as_mut() -> get inside box
	unsafe {
		coroutine::Coroutine::start(corout1.as_mut().get_raw_pointer());
	}






	// jetzt Schleife, wo jedes immer eins hochgezählt wird und dann gewechselt wird
}
