fn sqr(x: f64) -> f64 {
    x * x
    // body of the function has the value of the last expression
    // just like with if-as-an-expression
    // since there is no return statement, there should be no semicolon
}

fn add_one_by_ref(x: &i32) -> i32 {
    *x + 1
}

fn main() {
    let mut sum = 0.0;
    for i in 0..5 {
        let even_odd = if i & 1 == 0 { "even" } else { "odd" };
        println!("{} {}", even_odd, i);
        sum += sqr(i as f64);
    }
    println!("Sum of squre from 0 to 4: {}", sum);

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

fn modify(x: &mut f64) {
    *x = 1.0;
}
