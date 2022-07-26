#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]

#[macro_use]
extern crate lazy_static;

//use terminal::*;

mod panic;
mod qemu;
mod terminal;
mod test;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    loop {}
}
