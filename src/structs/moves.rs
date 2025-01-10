#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run() {
    /*
     *in a language where variables are always reference, `s2` becomes yet another reference
     *in C++, `s1` is a value, and it is copied to `s2`
     *but rust moves the value, it does not see strings as copyable ("does not implement the Copy trait")
     */
    // let s1 = "hello dolly".to_string();
    // let s2 = s1;
    // println!("{}", s1); // error -> borrowed after move

    /*
     * rust is a block-scoped language
     * variables only exist for the duration of their block-scoped
     * which introduce a rust-specific issue - a variable may appear to be in scope, but its value has moved
     */
    let a = 10;
    let b = "hello";
    {
        let c = "hello".to_string();
        // a,b and c are visible
    }
    // the string c is dropped
    // a,b are visible
    for i in 0..a {
        let b = &b[1..];
        // original b is no longer visible - it is shadowed, and confusing
    }
    // the slice b is dropped
    // i is _not_ visible!
    // loop variables are a little different, only visible in the loop block

    // let mut rs1 = &s1;
    // {
    //     let tmp = "hello world".to_string();
    //     rs1 = &tmp;
    // }
    // println!("{}", rs1); // error -> borrowed value does not live long enough
}
