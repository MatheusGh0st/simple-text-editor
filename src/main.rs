use std::env;
use std::process;
use std::fs;
use std::io;
use std::io::{Error};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    if !fs::metadata(filename).is_ok() {
        println!("Error file don't exist!");
        process::exit(1);
    }

    loop {
        println!("Enter a command |i: insert, r: read, d: delete, q: quit|: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input);

        match input.trim() {
            "i" => {}
            "r" => {}
            "d" => {}
            "q" => {
                println!("Exit");
                break;
            }
            _ => {
                println!("Invalid command| Try again");
            }
        }
    }

    Ok(())
}
