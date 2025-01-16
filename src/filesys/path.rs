use dirs;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::{env, io};

fn foo(_: &Path) {}

fn dump_dir(dir: &str) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let data = entry.metadata()?;
        let path = entry.path();
        if data.is_file() {
            // On Unix this is the way the opendir system call works
            // but on Windows you cannot iterate over a directory's contents without getting the metadata
            // So this is a reasonably elegant compromise that allows cross-platform code to be as efficient as possible
            if let Some(ex) = path.extension() {
                if ex == "rs" && data.len() > 1024 {
                    println!("{} length {}", path.display(), data.len());
                }
            }
        }
    }
    Ok(())
}

pub fn run() {
    let home = dirs::home_dir().expect("No home!");
    let mut path = PathBuf::new();
    // `PathBuf` is like `String` - owns a grwable set of characters
    // but with methods specialized to build up paths
    path.push(home);
    path.push(".cargo");
    if path.is_dir() {
        println!("{}", path.display());
    }
    // This might sound suspiciously like a form of inheritance, but the magic Deref trait works differently
    // It works just like it does with String/&str - a reference to PathBuf can be coerced into a reference to Path
    // ('Coerce' is a strong word, but this really is one of the few places where Rust does conversions for you)

    foo(&path);
    // `PathBuf` has an intimate relationship with OsString, which represents strings we get directly from the system
    // (There is a corresponding OsString/&OsStr relationship)

    let mut curr = env::current_dir().expect("cannot access current dir!");
    curr.push("src");
    curr.push("filesys");
    dump_dir(&curr.to_string_lossy()).expect("cannot dump dir");
    curr.pop();
    curr.pop();
    loop {
        curr.push("test.txt");
        if curr.is_file() {
            println!("gotcha {}", curr.display());
            break;
        } else {
            curr.pop();
        }
        if !curr.pop() {
            break;
        }
    } // pretty much hoiw **git** works when it wants to know what current repo is

    let file = env::args().nth(1).unwrap_or("test.txt".to_string());
    let file_path = Path::new(&file);
    match file_path.metadata() {
        Ok(data) => {
            println!("type {:?}", data.file_type());
            println!("size {}", data.len());
            // println!("perm {:?}", data.permissions());
            println!("perm {:o}", data.permissions().mode()); // '{:o}' for printing out in octal
            println!("modified {:?}", data.modified());
        }
        Err(e) => println!("error {:?}", e),
    }
}
