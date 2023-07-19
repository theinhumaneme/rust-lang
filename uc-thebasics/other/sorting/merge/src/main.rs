fn merge_sort(array: &mut [i32]) -> Vec<i32> {
    if array.len() > 1 {
        let mid = array.len() / 2;
        merge_sort(&mut array[..mid]);
        merge_sort(&mut array[mid..]);
        merge(array, mid);
    }
    array.to_vec()
}

fn merge(array: &mut [i32], mid: usize) {
    let left = array[..mid].to_vec();
    let right = array[mid..].to_vec();

    let mut mut_l = 0;
    let mut mut_r = 0;
    for val in array {
        if mut_r == right.len() || (mut_l < left.len() && left[mut_l] < right[mut_r]) {
            *val = left[mut_l];
            mut_l += 1;
        } else {
            *val = right[mut_r];
            mut_r += 1;
        }
    }
}

fn main() {
    let array = merge_sort(&mut vec![7, 5, 3, 8, 10, 0, 2]);

    println!("{:?}", array)
}

