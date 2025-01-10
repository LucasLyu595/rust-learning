fn add_mul(x: f64, y: f64) -> (f64, f64) {
    (x + y, x * y)
}

pub fn run() {
    let t = add_mul(2.0, 10.0);
    // can debug print
    println!("t {:?}", t);

    // can `index` the value
    println!("add {}, mul {}", t.0, t.1);

    // can _extract_ the value
    let (add, mul) = t;
    println!("add {}, mul {}", add, mul);

    // tuples may contain different types, which is the main difference from arrays
    #[allow(unused_variables)]
    let tuple = ("hello", 5, 5.5, true);

    // `Iterator` methods
    // `enumerate` is like the Python generator of the same name
    for (i, t) in ["zero", "one", "two"].iter().enumerate() {
        print!(" {} {};", i, t);
    }

    // `zip` combines two iterators into a single iterator of tuples
    let names = ["ten", "hundred", "thousand"];
    let nums = [10, 100, 1000];
    for (name, num) in names.iter().zip(nums.iter()) {
        print!(" {} {};", name, num);
    }
    println!();
}
