#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Into test store_fault, we will insert and invalid store operations...");
    println!("Kernel should kill this application");
    unsafe {
        core::ptr::null_mut::<u8>().write_volatile(0);
    }
    0
}
