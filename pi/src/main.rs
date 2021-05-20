fn main() {
    println!("Calculating Pi number...");
    let i = 0;
    let precision = 100_000_000 * 2;
    let mut sum:  f64 = 0.0;
    let mut sign: f64 = 1.0;
    let mut pi:   f64 = 0.0;
    let mut d:    f64 = 0.0;

    for i in 0..precision + 1{
        d = sign * (1.0 / (2.0 * i as f64 + 1.0));
        sum = sum + d;
        sign = -sign;
        pi = 4.0 * sum;
    }
    println!("{}, {}, {} ", i, pi, d);
}
