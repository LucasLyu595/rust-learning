pub fn run() {
    // `std::env::args` returns an iterator of the command line arguments as strings
    // including the program name
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        println!("Arguments: {:?}", args);
    } else {
        println!("No arguments provided");
    }

    // `expect` is like an`unwrap` with a readable error message
    let first = std::env::args().nth(1).expect("please supply an argument");
    let n: i32 = first.parse().expect("not an integer!");
    println!("You passed: {}", n);
}
