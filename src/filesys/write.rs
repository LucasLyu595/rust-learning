use std::fs::File;
use std::io::{self, Write};

fn write_out(f: &str) -> io::Result<()> {
    let mut out = File::create(f)?;
    writeln!(out, "answer is {}", 42)?;
    Ok(())
}

pub fn run() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    // another way of saying print!
    writeln!(handle, "answer is {}", 42).expect("write failed");
    // But there is a difference
    // `print!` locks stdout for each write - what you want for output
    // because without that locking multithreaded programs can mix up that output in interesting ways
    // But if you are pumping out a lot of text, then `write!` is going to be faster
    write_out("test.txt").expect("write failed");
    // If you care about performance, you need to know that Rust files are unbuffered by default
    // So each little write request goes straight to the OS, and this is going to be significantly slower
}
