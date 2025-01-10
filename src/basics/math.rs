fn sqr(x: f64) -> f64 {
    x * x
    // body of the function has the value of the last expression
    // just like with if-as-an-expression
    // since there is no return statement, there should be no semicolon
}

pub fn run() {
    let mut sum = 0.0;
    for i in 0..5 {
        let even_odd = if i & 1 == 0 { "even" } else { "odd" };
        println!("{} {}", even_odd, i);
        sum += sqr(i as f64); // explicit conversion
    }
    println!("Sum of squre from 0 to 4: {}", sum);
}
