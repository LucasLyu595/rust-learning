use std::f64::consts::PI;

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

pub fn run() {
    let answer = 42;
    let maybe_pi = PI;
    let s1 = answer.show();
    let s2 = maybe_pi.show();
    println!("show {}", s1);
    println!("show {}", s2);
}

/*
* `Display` controls how values are printed out with "{}" and is implemented like `Debug`
* as a useful side-effct, `ToString` is implemented for any type that implements `Display`
*
* `Clone` defines the method `clone`, and can simply be defined with `#[derive(Clone)]` if all the
* fields themselves implement `Clone`
*/
