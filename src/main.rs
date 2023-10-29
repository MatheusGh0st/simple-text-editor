use std::env;
use std::process;
use std::fs;
use std::io;
use std::io::{Error, Read, Write, Seek, SeekFrom};
use std::fs::OpenOptions;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    // Check if the filename exist
    if !fs::metadata(filename).is_ok() {
        println!("Error file don't exist!");
        process::exit(1);
    }

    loop {
        println!("Enter a command |i: insert, r: read, d: delete, q: quit|: ");
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);

        match input.trim() {
            "i" => {
                let _ = insert_file(&filename);
                break;
            }
            "r" => {
                let _ = read_file(&filename);
                break;
            }
            "d" => {
                let _ = delete_file(&filename);
                break;
            }
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

fn insert_file(file_path: &str) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&file_path)?;
    
    let metadata = file.metadata()?;

    // Take the last position in file
    let last_position = metadata.len();

    // Seek the pointer to the last position
    file.seek(SeekFrom::Start(last_position as u64))?;

    loop {
        println!("Insert the content: ");
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input)?;

        file.write_all(input.as_bytes())?;
        println!("Data written!");
        break;
    }

    Ok(())
}

fn read_file(file_path: &str) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(&file_path)?;
    
    let mut buffer = String::new();

    // Read the file and store in buffer
    file.read_to_string(&mut buffer)?;

    println!("Content: {}", buffer);

    Ok(())
}

fn delete_file(file_path: &str) -> Result<(), Error> {
    // Remove file
    fs::remove_file(file_path)?;

    println!("File deleted!");
    Ok(())
}