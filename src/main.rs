#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;

//use terminal::*;

mod panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
