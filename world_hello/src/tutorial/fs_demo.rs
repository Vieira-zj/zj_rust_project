use std::fs::File;
use std::io::Read;

// file io samples

pub fn read_file_sample_01() {
    let path = "/tmp/test/log.txt";
    match read_from_file(path) {
        Ok(s) => println!("read file:\n{}", s),
        Err(e) => println!("read {} error: {}", path, e),
    };
}

fn read_from_file(path: &str) -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

pub fn read_file_sample_02() {
    let path = "/tmp/test/log.txt";
    match std::fs::read_to_string(path) {
        Ok(s) => println!("read file:\n{}", s),
        Err(e) => println!("read {} error: {}", path, e),
    };
}