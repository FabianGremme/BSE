use crate::devices::cga;
use crate::kernel::cpu;
pub fn run() {
    for i in 1..10{
        cpu::disable_int();
        cga::setpos(1, i);
        cga::print_dec(i);
        cpu::enable_int();
    }
}
