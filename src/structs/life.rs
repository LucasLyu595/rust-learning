#[derive(Debug)]
struct A {
    // usually structs contain values, but often they contain references
    #[allow(dead_code)]
    s: &'static str,
    // from the point of view of Rust
    // it will not allow a reference to be stored without knowing its lifetime
    // the lifetime of a reference cannot be longer than the lifetime of the value
    //
    // String literals exist for the duration of the whole program
    // which is called the `static` lifetime
    // works for the special case of static strings but very restrictive
}

#[allow(dead_code)]
fn how(i: i32) -> &'static str {
    match i {
        0 => "none",
        1 => "one",
        _ => "many",
    }
}

#[derive(Debug)]
struct B<'b> {
    // specify that the lifetime of the reference is at least as long as that of the struct itself
    #[allow(dead_code)]
    s: &'b str,
}

// there is no way that this could safely work, because `string` will be dropped when the function ends
// and no reference to `string` can outlast it
// fn makes_b() -> B<'static> {
//     let string = "I'm a little string".to_string();
//     B { s: &string }
// }
// sometimes it seems like a good idea for a struct to contain a value and a reference that borrows
// from that value. However, it's basically impossible because struct must be **movable**

pub fn run() {
    let a = A { s: "hello dammit" };

    let s = "I'm a little string".to_string();
    let b = B { s: &s };

    println!("{:?}", a);
    println!("{:?}", b);
}
