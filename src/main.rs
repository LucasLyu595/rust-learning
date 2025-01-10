mod basics;

use std::f64::consts;

fn main() {
    basics::math::run();
    println!("-----------------");

    basics::reference::run();
    println!("-----------------");

    // use a constant from the standard library
    let x = 2.0 * consts::PI;
    let abs_diff = (x.cos() - 1.0).abs();
    assert!(abs_diff < 1e-10);

    basics::arrays::run();
    println!("-----------------");

    basics::slices::run();
    println!("-----------------");

    basics::vectors::run();
    println!("-----------------");

    basics::iter::run();
    println!("-----------------");

    basics::strings::run();
    println!("-----------------");

    // basics::args::run();
    // println!("-----------------");

    basics::matching::run();
    println!("-----------------");

    basics::files::run();
}
