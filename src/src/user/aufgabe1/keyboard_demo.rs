/*
use crate::devices::cga as cga;  // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::devices::key as key;      // shortcut for key
use crate::devices::keyboard as keyboard;  // shortcut for keyboard
use crate::keyboard::set_global_repeat_rate;



pub fn run() {

    Hier muss Code einfgeügt werden
        
   // 'key_hit' aufrufen und Zeichen ausgeben
    set_global_repeat_rate(31, 3);
    for i in 1..200{
        let zeichen = keyboard::key_hit().get_ascii();
        if zeichen != 0{
            print!("{}", zeichen as char);
        }

    }


}
*/