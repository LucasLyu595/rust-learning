fn dump(arr: &[i32]) {
    println!("arr is {:?}", arr);
}

pub fn run() {
    // Vector
    // re-sizeable array, after being declared as mutable
    let v = vec![1, 2, 3];

    dump(&v);

    let first = v[0];
    let maybe_first = v.first();

    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);
}
