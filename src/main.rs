#![no_std]
#![no_main]

use wildnix::*;

#[wildnix::main]
fn main() {
    println!("Hello, WildNIX!");
    println!("This program is running in userspace!");
    println!("WildNIX Init-rs v{}", env!("CARGO_PKG_VERSION"));
    println!("Type 'help' for a list of commands.");
    
    loop {
        print!("root@wildnix # ");
        let mut input = [0u8; 256];
        let len = read_line(&mut input);
        
        // Ensure we have valid UTF-8 and trim whitespace
        let command = match core::str::from_utf8(&input[..len]) {
            Ok(s) => s.trim(),
            Err(_) => {
                println!("Error: invalid UTF-8 input");
                continue;
            }
        };
        
        // Skip empty commands
        if command.is_empty() {
            continue;
        }

        match command {
            "help" => {
                println!("Available commands:");
                println!("  help - Show this help message");
                println!("  echo <text> - Echo the provided text");
                println!("  exit - Exit the program");
            }
            cmd if cmd.starts_with("echo ") => {
                let text = &cmd[5..];
                println!("{}", text);
            }
            "exit" => {
                println!("Exiting...");
                unsafe { syscall1(SYS_EXIT, 0) };
            }
            _ => {
                println!("Unknown command: '{}'", command);
            }
        }
    }
}
