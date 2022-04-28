#![no_std]
#![no_main]
use user_lib::*;

#[allow(unused)]
#[no_mangle]
fn main() -> i32{
    println!("hello from user mode");
    0
}
