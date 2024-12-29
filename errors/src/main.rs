use std::fs::File;
use std::io::{ErrorKind, Write};
fn main() {
    let greeting_file_result = File::open("hello.txt");// File doesn't exist

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {e:?}"),
            },
            some_error => {
                panic!("Problem opening the file: {some_error:?}");
            }
        }
    };

    match greeting_file.write("Hello, World!\n".as_bytes()) {
        Ok(n) => println!("Wrote {n:05} bytes to file"),
        Err(e) => eprintln!("Could not write to file => \"{e:?}\""),
    }
}
