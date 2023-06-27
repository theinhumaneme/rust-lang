fn main() {
    let vector: Vec<u32> = vec![1, 3, 5, 7, 9];
    let vector = vector.iter().map(|x| x * 10).collect::<Vec<u32>>();
    println!("{:?}", vector);
}
