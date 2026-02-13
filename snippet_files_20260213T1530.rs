// snippet_files_20260213T1530.rs
// Topic: Reading and writing files in Rust
// Demonstrates basic file I/O: create, read, append, and delete.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    let file_path = "example.txt";

    // Write initial content
    {
        let mut file = File::create(file_path)?;
        file.write_all(b"Hello, Rust file I/O!")?;
    }

    // Read content
    let mut content = String::new();
    {
        let mut file = File::open(file_path)?;
        file.read_to_string(&mut content)?;
    }
    println!("Read from file: {}", content);

    // Append additional text
    {
        let mut file = OpenOptions::new().append(true).open(file_path)?;
        file.write_all(b"\nAppending a second line.")?;
    }

    // Read again to see appended content
    {
        let mut file = File::open(file_path)?;
        content.clear();
        file.read_to_string(&mut content)?;
    }
    println!("After append: {}", content);

    // Delete the file
    fs::remove_file(file_path)?;
    println!("File deleted.");

    // Additionally: create a directory and a file inside it
    let dir = "temp_dir";
    fs::create_dir_all(dir)?;
    let subfile = format!("{}/nested.txt", dir);
    fs::write(&subfile, "Nested content")?;
    println!("Created nested file: {}", subfile);

    // Clean up: remove directory recursively
    fs::remove_dir_all(dir)?;
    println!("Cleaned up temp_dir.");

    Ok(())
}

/*
To run:

cargo build --release
./target/release/snippet_files_20260213T1530

Or simply:

cargo run --bin snippet_files_20260213T1530.rs
*/
