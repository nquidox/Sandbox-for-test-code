use rand::Rng;
use std::io;


fn main() {
    println!("The generator of random numbers \nver 0.0.1 \n\
    Inspired by qbasic generator by V.N. Kolchin");

    while true{
        let mut random = rand::thread_rng();
        let x: f64 = (random.gen_range(0.0..1.0) * 6.0) + 1.0;
        let y: f64 = (random.gen_range(0.0..1.0) * 6.0) + 1.0;
        let b: f64 = (random.gen_range(0.0..1.0) * 6.0) + 1.0;
        let c: f64 = (random.gen_range(0.0..1.0) * 6.0) + 1.0;
        let s: f64 = (random.gen_range(0.0..1.0) * 6.0) + 1.0;
        let d: f64 = (random.gen_range(0.0..1.0) * 6.0) + 1.0;

        println!("Press 1 to generate a number, press 2 to exit.");
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input = input.trim().to_string();
        if input == "1"{
            println!("{:.0}{:.0}{:.0}{:.0}{:.0}{:.0}", x, y, b, c, s, d);
        }
        else {break}
    }
}
