fn sot(nums: &mut Vec<i32>) {
    if nums.len() < 1 {
        return;
    }
    let mut sums: Vec<i32> = vec![];
    for i in 1..nums.len() {
        sums.push(nums[i - 1] + nums[i]);
    }
    sot(&mut sums);
    println!("{:?}", nums)
}

fn main() {
    sot(&mut vec![1, 2, 3, 4, 5]);
}
