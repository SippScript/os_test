#![no_std] // tells program not to include standard lib
#![no_main] // tells the program to disable default entry point of program ("fn main(){...}")

// Panic Handler
use::core::panic::PanicInfo;

// function to be called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

// function to replace our "fn main(){...}"
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop{}
}



