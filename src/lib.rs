use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::env;


pub fn get_input() -> Option<String> {
    env::args()
        .last()
        .and_then(|filename| read_file(&filename).ok())
}

fn read_file(filepath: &str) -> io::Result<String> {
    let file = File::open(filepath)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}
