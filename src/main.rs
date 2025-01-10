mod basics;
mod structs;

use std::f64::consts;

fn basic_run() {
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

fn struct_run() {
    // structs::moves::run();
    // println!("-----------------");

    structs::tuples::run();
    println!("-----------------");

    structs::stru::run();
    println!("-----------------");

    structs::life::run();
    println!("-----------------");

    structs::traits::run();
    println!("-----------------");

    structs::genfn::run();
}

fn main() {
    basic_run();
    println!("-----------------");
    struct_run();
}
