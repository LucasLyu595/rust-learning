// read as: slice of i32
fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for val in values {
        res += val;
    }
    res
}

pub fn run() {
    let arr = [10, 20, 30, 40];
    // pay attention to the `&` symbol
    let res = sum(&arr);
    // the relation between Rust arrays and slices is similar to the relation between C arrays and pointers
    // except for
    //   1. Rust slices keep track of their length
    // `&` pronounce 'borrow', anything borrowed remains owned by the original owner
    println!("sum {}", res);

    let slice = &arr[1..];
    println!("slice {:?}", slice);
    // open range slice looks similar to Python slices but with big difference:
    //   - a copy of the data is never made
    // size of the slice is only known at runtime

    // Optional value
    let first = slice.first();
    let maybe_last = slice.get(5); // use `get` instead of `[]` to avoid panic
    println!("first {:?}", first);
    println!("maybe_last {:?}", maybe_last);
    // Option type may be either `Some` or `None`
    println!("first {} {}", first.is_some(), first.is_none());
    println!(
        "maybe_last {} {}",
        maybe_last.is_some(),
        maybe_last.is_none()
    );

    // formatting traits (like `Debug` and `Display`) are automatically implimented for both the
    // value and its reference, so the '*' below is unnecessary
    println!("first {}", *first.unwrap()); // the precise type in this case inside 'Some' is '&i32'
    let last = *maybe_last.unwrap_or(&-1);
    // equivalent to
    // ```
    // let last = if maybe_last.is_some() {
    //   *maybe_last.unwrap()
    //   } else {
    //     -1
    //   }
    // ```
    println!("last {}", last);
}
