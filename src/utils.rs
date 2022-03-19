use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn read_to_vec(path: &str) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;
    let mut reader = io::BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    Ok(buf)
}
