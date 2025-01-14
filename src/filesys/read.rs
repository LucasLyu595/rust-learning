use std::fs::File;
use std::io;
use std::io::prelude::*; // easiest way to make sure all following traits are visible

// `fs::File` implements `io::Read`, which is the trait for anything readable ('raw' read, with no buffering)
// defines a `read` method which will fill a slice of `u8` with bytes
// being the only *required* method of the trait, get some provided method for free, e.g. `Iterator`
// `read_to_end` to fill a vector of bytes with contents from the readable
// `read_to_string` to fill a string - which must be UTF-8 encoded
//
// for buffered reading, there is the `io::BufRead` trait which gives us `read_line` and a `lines` iterator
// `io::BufReader` will provide an implementation of `io::BufRead`
#[allow(dead_code)]
fn read_all_lines1(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        // not the most efficient way to read all lines, with a new
        // string allocated for eath line
        let line = line?;
        println!("{}", line); // strings can go wrong during the iteration
    }

    Ok(())
}

#[allow(dead_code)]
fn read_all_lines2(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let mut reader = io::BufReader::new(file);

    let mut buf = String::new();
    while reader.read_line(&mut buf)? > 0 {
        // {
        let line = buf.trim_end(); // returned line includes the linefeed
        println!("{}", line);
        // } // use a block to control the borrow of `line` to prevent access `line` after the buffer cleared
        buf.clear(); // clearing the string does not free allocated memory
    }

    Ok(())
}

struct Lines<R> {
    reader: io::BufReader<R>,
    buf: String,
}

impl<R: Read> Lines<R> {
    fn new(r: R) -> Self {
        Self {
            reader: io::BufReader::new(r),
            buf: String::new(),
        }
    }

    fn next(&mut self) -> Option<io::Result<&'_ str>> {
        // this signature, with the lifetime, is incompatible with the interface of `Iterator`
        self.buf.clear();
        match self.reader.read_line(&mut self.buf) {
            Ok(nbytes) => {
                if nbytes == 0 {
                    None
                } else {
                    Some(Ok(self.buf.trim_end()))
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}

fn read_all_lines3(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let mut lines = Lines::new(file);
    while let Some(line) = lines.next() {
        println!("{}", line?);
        // it's tempting, but throwing away a possible error
        // this loop will silently stop whenever an error occurs
    }

    Ok(())
}

pub fn run() {
    let filename = "test.txt";
    read_all_lines3(filename).expect("Could not read file");
}
