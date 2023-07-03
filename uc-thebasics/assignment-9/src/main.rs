macro_rules! op {
    ($val1:expr,$val2:expr,$val3:expr) => {
        match $val3{
            1 => $val1 + $val2,
            2 => $val1 - $val2,
            3 => $val1 * $val2,
            4 => $val1 / $val2,
            5 => $val1 % $val2,
            _ => -1,
        }
    };
}
fn main() {
    println!("Hello world");
    println!("{}", op!(5, 2, 1)); //should print 7

    println!("{}", op!(5, 2, 2)); //should print 3

    println!("{}", op!(5, 2, 3)); //should print 10

    println!("{}", op!(5, 2, 4)); //should print 2

    println!("{}", op!(5, 2, 5)); //should print 1

    println!("{}", op!(5, 2, 6)); //should print -1
}

