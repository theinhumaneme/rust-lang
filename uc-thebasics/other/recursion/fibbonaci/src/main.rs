fn fibbonaci(num: i128) -> i128 {
    if num == 0 {
        0
    } else if num == 1 {
        1
    } else {
        return fibbonaci(num - 1) + fibbonaci(num - 2);
    }
}

fn main() {
    println!("{}", fibbonaci(40));
}
