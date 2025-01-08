mod utils;

use std::f64::consts;

fn main() {
    utils::math::run();

    utils::reference::run();

    // use a constant from the standard library
    let x = 2.0 * consts::PI;
    let abs_diff = (x.cos() - 1.0).abs();
    assert!(abs_diff < 1e-10);

    utils::arrays::run();

    utils::slices::run();
}
