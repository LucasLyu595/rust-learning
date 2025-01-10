fn add_one_by_ref(x: &i32) -> i32 {
    *x + 1
}

fn modify(x: &mut f64) {
    *x = 1.0;
}

pub fn run() {
    // immutable reference
    let i = 10;
    let res1 = add_one_by_ref(&i);
    let res2 = add_one_by_ref(&41);
    println!("{} {}", res1, res2);
    println!("i is {}", i);

    // mutable reference
    let mut res = 0.0;
    modify(&mut res);
    println!("res is {}", res);
}
