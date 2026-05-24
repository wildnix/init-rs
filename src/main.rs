#![no_std]
#![no_main]

use libwildnix::*;

#[libwildnix::main]
fn main() {
    println!("Hello, WildNIX!");
    println!("This program is running in userspace!");
}
