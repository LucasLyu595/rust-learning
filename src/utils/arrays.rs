pub fn run() {
    // array is mutbale by default, but fixed size
    // type of an array includes its size
    let arr = [10, 20, 30, 40];
    // [i32; 4]
    // not great for passing around as function arguments, use slices instead
    let first = arr[0];
    println!("first {}", first);

    for i in 0..arr.len() {
        println!("[{}] = {}", i, arr[i]);
    }
    println!("length {}", arr.len());
}
