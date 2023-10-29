use std::env;
use std::io::{Error};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    Ok(())
}
