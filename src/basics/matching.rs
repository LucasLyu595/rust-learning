struct Point {
    x: f32,
    y: f32,
}

fn match_tuple(t: (i32, String)) {
    let text = match t {
        (0, s) => format!("zero {}", s),
        (1, ref s) if s == "hello" => "hello one".to_string(),
        // why not just use `if s == "hello"`?
        // match is exact business, and the compiler will complain
        tt => format!("no match {:?}", tt),
    };
    println!("{}", text);
}

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

    let t = (10, "hello".to_string());
    match_tuple(t);
    // let (ref n, ref s) = t;
    // n is &i32, s is &String
    // explicitly borrowing the value
    // let (n, s) = t;
    // n is i32, s is String

    // destructuring works with structs too
    let p = Point { x: 1.0, y: 2.0 };
    #[allow(unused_variables)]
    let Point { x, y } = p;
    // p still lives, since x and y can and will be copied

    let ot = Some((2, "hello".to_string()));
    if let Some((_, ref s)) = ot {
        assert_eq!(s, "hello");
    }
    // just borrowed the string, no 'destructuring'

    // if let Ok(n) = "42".parse() {
    if let Ok(n) = "42".parse::<i32>() {
        println!("n is {}", n);
    }
}
