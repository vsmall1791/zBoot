//main.rs

//Disables standard library
#![no_std]
//Sets an entry point
#![no_main]
#![allow(non_snake_case)]


//Defines a panic handler
use core::panic::PanicInfo;

//Call this function on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}
//panic info contains line and file source of panic


//We need a language item
/*
-Used by the compiler
Either create one(buggy) or disable unwinding(optimal)

Disabled by adding this code to the Cargo.toml file:

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"

sets the panic strategy default to abort for dev and release profiles
*/

#[no_mangle]//Forces the compiler to output a function name '_start'
pub extern "C" fn _start() -> !{
    loop{}
}
