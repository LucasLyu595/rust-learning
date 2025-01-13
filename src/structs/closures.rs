use std::f64::consts::PI;
struct MyAnonymousClosure1<'a> {
    m: &'a f64,
    c: &'a f64,
}

// lifetime anotation elision
impl MyAnonymousClosure1<'_> {
    // impl<'a> MyAnonymousClosure1<'a> {
    fn call(&self, x: f64) -> f64 {
        self.m * x + self.c
    }
}

fn apply<F>(x: f64, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    f(x)
}

// calling a closure is a method call
// three kinds of functions traits correspond to the three kinds of functions
// `Fn` struct passed as `&self`
// `FnMut` struct passed as `&mut self`
// `FnOnce` struct passed as `self`
fn mutable<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}

pub fn run() {
    // no explicit types - everything is deduced
    // but the first call fixes the type of argument
    let f = |x| x * x;
    let res = f(10);
    println!("res: {}", res);

    let m = 2.0;
    let c = 1.0;
    // you cannot do this with the explicit fn form
    // does not know about variables in the enclosing scope
    // closure has borrowed `m` and `c` from its context
    let lin = |x| m * x + c;
    println!("res {} {}", lin(1.0), lin(2.0));
    let lin1 = MyAnonymousClosure1 { m: &m, c: &c };
    println!("res {} {}", lin1.call(1.0), lin1.call(2.0));
    // the type of `lin`, only rustc knows
    // under the hood, a closure is a *struct* that is callable ('implements the call operator')
    // therefore it has a lifetime
    // all closures are unique types, but they have traits in common
    // even though we don't know the exact type, we know the generic constraint
    let res1 = apply(3.0, lin); // `apply` ate the closure, e.g. it is moved and not usable anymore
    let res2 = apply(PI, |x| x.sin());
    println!("res1: {}, res2: {:.1}", res1, res2);

    let mut s = "world";
    mutable(|| {
        s = "hello";
    });
    assert_eq!(s, "hello");

    let mut changer = || s = "world";
    changer();
    // the mutable borrow of `s` ends after the closure is called
    assert_eq!(s, "world");

    let name = "dolly".to_string();
    let cname = name.clone();
    let age = 42;
    // move closure - takes ownership of the variables
    // needed when original context no longer exists
    // classic case is when creating a thread
    // moved closure does not borrow, so does not have a lifetime
    let c = move || {
        println!("name: {}, age: {}", cname, age);
    };
    c();
    println!("name {}", name);

    let tuples = [(10, "ten"), (20, "twenty"), (30, "thirty"), (40, "forty")];
    let iter = tuples.iter().filter(|t| t.0 > 20).map(|t| t.1);
    for name in iter {
        print!("{} ", name);
    }
    println!();
}
