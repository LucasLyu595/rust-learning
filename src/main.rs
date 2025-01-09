mod utils;

use std::f64::consts;

fn main() {
    utils::math::run();
    println!("-----------------");

    utils::reference::run();
    println!("-----------------");

    // use a constant from the standard library
    let x = 2.0 * consts::PI;
    let abs_diff = (x.cos() - 1.0).abs();
    assert!(abs_diff < 1e-10);

    utils::arrays::run();
    println!("-----------------");

    utils::slices::run();
    println!("-----------------");

    utils::vectors::run();
    println!("-----------------");

    utils::iter::run();
    println!("-----------------");

    utils::strings::run();
}
