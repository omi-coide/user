#![no_std]
#![no_main]
use user_lib::*;

#[no_mangle]
fn main() -> i32 {
    println!("this should die; Write to NULL");
    unsafe {
        core::ptr::null_mut::<u8>().write_volatile(0);
    };
    0
}
