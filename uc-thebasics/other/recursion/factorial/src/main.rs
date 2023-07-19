fn factorial(num: i128) -> i128 {
    if num > 1 {
        num * factorial(num - 1)
    } else {
        1
    }
}

fn main() {
    println!("{}", factorial(30));
}
