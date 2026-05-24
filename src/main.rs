#![no_std]
#![no_main]

use wildnix::*;

#[wildnix::main]
fn main() {
    println!("Hello, WildNIX!");
    println!("This program is running in userspace!");
}
