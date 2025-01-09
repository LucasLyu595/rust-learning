pub fn run() {
    let multilingual = "Hi! ¡Hola! привет!";
    // COMPARISON
    // 1.
    // let maybe = multilingual.find('п');
    // if maybe.is_some() {
    //     let hi = &multilingual[maybe.unwrap()..];
    //     println!("Russian hi: {}", hi);
    // }
    // 2.
    match multilingual.find('п') {
        Some(idx) => {
            // save an extra variable to store the `option`
            let hi = &multilingual[idx..];
            println!("Russian hi {}", hi);
        }
        None => println!("couldn't find the greeting, Товарищ"),
    };
    // 3.
    // if you are not insterested in the `None` case, you can use `if let`
    // if let Some(idx) = multilingual.find('п') {}

    // let first = std::env::args()
    //     .nth(1)
    //     .expect("please supply an integer argument");
    // let n: i32 = first.parse().expect("this is not an integer!");
    let n = 2;
    let text = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "many", // `default` - fall-back case, must have othersize error
    };
    print!("{:?} is {}", n, text);
    // `match` can also on ranges, [DEPRECATED] three dots `...` are inclusive, use `..=` instead
    let text = match n {
        0..=3 => "small",
        4..=7 => "medium",
        _ => "large",
    };
    println!(" and {}", text);
}
