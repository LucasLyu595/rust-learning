fn main() {
    let mut sum = 0;
    for i in 0..5 {
        let even_odd = if i & 1 == 0 { "even" } else { "odd" };
        println!("{} {}", even_odd, i);
        sum += i;
    }
    println!("Sum: {}", sum);
}
