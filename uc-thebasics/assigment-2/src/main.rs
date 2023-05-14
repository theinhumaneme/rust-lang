fn main() {

    // create a vector with some values and pass to the function
    let mut vector =vec![1,3,5,7];
    let val:bool = take_val(&vector);
    println!("{}",val);
    vector.push(15);
    println!("{:?}",vector);
    add_two(5)
}

fn take_val(vector: &Vec<i8>) -> bool{
    if vector[0] == 1{
        true
    }
    else{
        false
    }
}


fn add_two(number: i8){
    println!("{}",number+2);
}
