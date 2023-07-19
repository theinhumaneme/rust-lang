fn swap<'a>(nums: &'a mut Vec<i32>, i: usize, j: usize) {
    let temp = nums[i];
    nums[i] = nums[j];
    nums[j] = temp;
}
fn selection_sort(nums: &mut Vec<i32>, order: bool) {
    for i in 0..nums.len() {
        let val = nums[i];
        for j in (i + 1)..nums.len() {
            if order == true {
                if val < nums[j] {
                    swap(nums, i, j);
                }
            } else {
                if val > nums[j] {
                    swap(nums, i, j);
                }
            }
        }
    }
    println!("{:?}", nums);
}

fn main() {
    selection_sort(
        &mut vec![1, 3, 4, 5, 6, 7, 8, 22, 3, 4, 4, 5, 6, 6, 7],
        true,
    )
}
