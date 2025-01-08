mod utils;

use crate::utils::reference;
use std::f64::consts;

fn main() {
    let mut sum = 0.0;
    for i in 0..5 {
        let even_odd = if i & 1 == 0 { "even" } else { "odd" };
        println!("{} {}", even_odd, i);
        sum += utils::math::sqr(i as f64);
    }
    println!("Sum of squre from 0 to 4: {}", sum);

    // immutable reference
    let i = 10;
    let res1 = reference::add_one_by_ref(&i);
    let res2 = reference::add_one_by_ref(&41);
    println!("{} {}", res1, res2);
    println!("i is {}", i);

    // mutable reference
    let mut res = 0.0;
    reference::modify(&mut res);
    println!("res is {}", res);

    // use a constant from the standard library
    let x = 2.0 * consts::PI;

    let abs_diff = (x.cos() - 1.0).abs();

    assert!(abs_diff < 1e-10);
}
