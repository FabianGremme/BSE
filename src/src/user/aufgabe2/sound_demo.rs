
use crate::devices::pcspk;
use crate::cga_print::print;


pub fn run() {
 
   // Hier muss Code eingefuegt werden
    println!("tetris");
    pcspk::tetris();
    println!("aerodynamic");
    pcspk::aerodynamic();
    print!("Sound done!");
}
