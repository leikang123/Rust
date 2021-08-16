use std::fs::File;
use std::io;
use std::io::Read;

fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let str_file = read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                println!("No such file");
            }
            _ => {
                println!("Cannot read the file");
            }
        },
    }
}
