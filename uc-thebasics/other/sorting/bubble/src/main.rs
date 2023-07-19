fn swap<'a>(nums: &'a mut Vec<i32>, i: usize, j: usize) {
    let temp = nums[i];
    nums[i] = nums[j];
    nums[j] = temp;
}
fn selection_sort(nums: &mut Vec<i32>, order: bool) {
    for i in 0..(nums.len() - 1) {
        for j in 0..(nums.len() - i - 1) {
            if order == true {
                if nums[j + 1] < nums[j] {
                    swap(nums, j + 1, j);
                }
            } else {
                if nums[j + 1] > nums[j] {
                    swap(nums, j + 1, j);
                }
            }
        }
    }
    println!("{:?}", nums);
}

fn main() {
    selection_sort(&mut vec![7, 5, 3, 8, 10, 0, 2], false)
}
