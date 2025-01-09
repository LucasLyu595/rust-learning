fn dump(arr: &[i32]) {
    println!("arr is {:?}", arr);
}

pub fn run() {
    // Vector
    // re-sizeable array, after being declared as mutable
    let mut v1 = vec![1, 2, 3, 4];

    dump(&v1);

    let first = v1[0];
    let maybe_first = v1.first();

    println!("v1 is {:?}", v1);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);

    // remove values from the end of a vector
    v1.pop();

    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);
    // compare with each other and with slices by value
    assert_eq!(v1, v2);

    // extend a vector using any compatible iterator
    v2.extend(0..2);
    println!("v2 is {:?}", v2);

    // insert values and remove values at arbitrary positions with `insert` and `remove`
    // not as efficient as pushing and popping

    // vectors have a size and a capacity
    // if you `clear` a vector, its size become zero, but capacity remains the same

    // vectors can be sorted, and then duplicates can be removed, operated in-pkace
    // if you want to make a copy first, use `clone` or `to_vec`
    let mut v3 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    v3.sort();
    v3.dedup();
    println!("v3 is {:?}", v3);
}
