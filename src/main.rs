use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("disk.rs")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    println!("{:?}", read_username_from_file());
}
