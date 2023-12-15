use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn call() {
    let file = File::open("files/day_1.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hello, world!");
    Ok(())
}
