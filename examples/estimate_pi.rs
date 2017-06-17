// A very inefficient way to approximate PI using random number.
// 
// with const N: u64 = 100_000_000_000;
// ~14mn
// pi    ~= 3.14159183515999985
// error ~= 0.00000081842979327    

extern crate pcgrng;
use std::f64;

fn main() {
    let mut rng = pcgrng::PCG32::seed(0, 0);

    const N: u64 = 1_000_000;
    let mut nb_inside = 0;
    for _ in 0..N+1 {
        let (x,y) = (rng.next_f64(), rng.next_f64());
        let d = x*x + y*y; // check if randomly select point is inside the circle
        nb_inside += (d < 1.0) as u64;
    };
    let pi_estimate = (4 * nb_inside) as f64 / (N as f64);
    println!("pi    ~= {:.17}", pi_estimate);
    let error = (f64::consts::PI - pi_estimate).abs();
    println!("error ~= {:.17}", error);
}
