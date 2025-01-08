pub fn run() {
    // array
    // - has fixed size
    // - type of an array includes its size
    let arr = [10, 20, 30, 40];
    // arr: [i32; 4]
    // - not great for passing around as function arguments, use slices instead
    // - contains values of *only one type*, to keep effiecient memory layout
    println!("ints: {:?}", arr);
    // debug pinting with :? is only available for types that implement the `std::fmt::Debug` trait
    let first = arr[0];
    println!("first {}", first);
    println!("length {}", arr.len());

    let ints_ints = [[1, 2], [30, 40]];
    println!("ints_ints: {:?}", ints_ints);
}
