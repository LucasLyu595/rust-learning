fn dump<T>(value: &T)
where
    T: std::fmt::Debug,
{
    println!("value: {:?}", value);
}

fn sqr<T>(x: T) -> T::Output
where
    // `concept` in C++ are like trait-constrained type parameters in Rust
    T: std::ops::Mul + Copy,
{
    x * x
}
// monomorphic, in constract to polymorphic
// the body of the function is compiled for each unique type
// produce faster code, and can often be inlined
// the downside is that the code size grows with each new type, which can result in code bloat
// while with *polymorphic* functions, the same machine code works with each matching type, dynamically dispatching the correct implementation

pub fn run() {
    let n = 42;
    dump(&n);
    dump(&sqr(n));
}
