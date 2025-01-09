fn dump(s: &str) {
    println!("str {}", s);
}

fn array_to_str(arr: &[i32]) -> String {
    let mut res = '['.to_string();
    for v in arr {
        res += &v.to_string();
        res.push(',');
    }
    res.pop();
    res.push(']');
    res
}

pub fn run() {
    // string literals are of type &str, not String
    // similar to the distinction between `const char*` and `std::string` in C++
    // under the hood,`String` is basically a `Vec<u8>`, but those bytes must be valid UTF-8
    let txt = "hello dolly";
    let mut s = txt.to_string();
    dump(txt);
    dump(&s);

    // like a vector, you can push/pop/append strings
    s.push(' ');
    s.push_str("hello");
    s += " world!";
    s.pop();

    println!("String: {}", s);

    let arr = array_to_str(&[10, 20, 30]);
    let res = format!("hello {}", arr);
    println!("{}", res);

    let res_s = &res[1..];
    println!("slice {:?}", res_s);

    // but you cannot index strings
    // One True Encoding, UTF-8, where a 'character' may be  a number of bytes
    // `char` type is a 4-byte Unicode code point. `String`s are **not** arrays of chars
    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() {
        print!("{:?} ", ch);
    }
    println!();
    println!("len {}", multilingual.len());
    println!("char count {}", multilingual.chars().count());

    // if you use a method like `find`, you will get a valid index(if found)
    let maybe = multilingual.find('п');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi: {}", hi);
    }

    // `split_whitespace` method returns an iterator
    // `collect` method is very general, so needs some clues about what it is collecting
    // hence the explicit type annotation
    let text = "the red fox and the lazy dog";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("{:?}", words);

    // `filter` method takes a closure that returns a boolean
    let stripped: String = text.chars().filter(|ch| !ch.is_whitespace()).collect();
    println!("{:?}", stripped);
}
