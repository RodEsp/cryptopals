use std::{fs::OpenOptions, io::prelude::*};

// TODO: Automatically create the log.txt file if it does not exist.
// Right now it needs to be created manually before using this function
pub fn _write(message: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("log.txt")
        .unwrap();

    if message != "" {
        if let Err(e) = writeln!(file, "{}", message) {
            eprintln!("Couldn't write to file:\n {}", e);
        }
    }
}

pub fn _clear() {
    todo!();
}
