#[allow(unused_imports)]
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

fn good_or_bad(good: bool) -> Result<i32, String> {
    if good {
        Ok(42)
    } else {
        Err("bad".to_string())
    }
}

#[allow(dead_code)]
fn read_to_string1(filename: &str) -> Result<String, io::Error> {
    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let path = Path::new(filename);
    if let Some(fname) = path.file_name() {
        println!("Reading file {:?}", fname);
    }

    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
} // not elegant

// `std::io` module defines a type alias `io::Result<T>` which is exactly the same as `Result<T, io::Error>`
fn read_to_string2(filename: &str) -> io::Result<String> {
    // let mut file = try!(File::open(filename));
    // this would do the same but usually in older code before 2017
    let mut file = File::open(filename)?;
    // `?` operator does almost exactly the same thing as the `match` in the previous example
    // if the result was an error, then it will immediately return that error
    // otherwise, it returns the `Ok` result, which we still need to wrap up as a result

    let path = Path::new(filename);
    if let Some(fname) = path.file_name() {
        println!("Reading file {:?}", fname);
    }

    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

pub fn run() {
    match good_or_bad(true) {
        Ok(n) => println!("Cool, I got {}", n),
        Err(e) => println!("Huh, I just got {}", e),
    }

    // let first = env::args().nth(1).expect("Please supply a file name");
    let first = "test.txt";
    let text = read_to_string2(first).expect("bad file man!");
    println!("file had {} bytes", text.len());
}
