use rayon::prelude::*;
use std::time::Instant;
fn fib_recursive(n: u64) -> u64 {
    if n < 2 {
        return 1;
    }
    fib_recursive(n - 1) + fib_recursive(n - 2)
}
fn fibonnaci_join(n: u64) -> u64 {
    if n < 2 {
        return 1;
    }
    let (a, b) = rayon::join(|| fib_recursive(n - 1), || fib_recursive(n - 2));
    a + b
}
fn main() {
    let start = Instant::now();
    let x = fib_recursive(47);
    let duration = start.elapsed();
    println!("{:?},{:?}", x, duration);
    let start = Instant::now();
    let y = fibonnaci_join(47);
    let duration = start.elapsed();
    println!("{:?},{:?}", y, duration);
}
