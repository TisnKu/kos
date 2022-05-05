use core::panic::PanicInfo;
use crate::println;
use crate::sbi::shut_down;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("Panicked at: {}:{} {}", location.file(), location.line(), info.message().unwrap());
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shut_down()
}
