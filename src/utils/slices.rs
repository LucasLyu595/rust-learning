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
}
