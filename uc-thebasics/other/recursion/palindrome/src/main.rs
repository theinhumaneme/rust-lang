fn palindrome(hector: Vec<i32>) -> bool {
    for i in 0..hector.len() / 2 {
        if hector[i] == hector[hector.len() - i - 1] {
            continue;
        } else {
            return false;
        }
    }
    return true;
}

fn main() {
    println!("{}", palindrome(vec![1, 2, 3, 4, 3, 2, 1]));
}
