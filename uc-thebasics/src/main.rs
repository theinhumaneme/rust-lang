fn main(){

    let val1 = 5;
    let val2 = 22;
    let ans= val1%val2;
    println!("{ans}");

    let mut vec1 = vec![2,4,6,8,10];
    vec1.remove(4);
    vec1.push(12);
    println!("{:?}",vec1);

    let hello = String::from("Hello");
    println!("{:?}",concat_string(&hello));
    println!("{}",hello);
    control_flow(50);
}

fn concat_string(arg: &String) -> String{
    let final_word= arg.to_string()+ " World";
    final_word
}

fn control_flow(number: i32){
    if number > 25{
    println!("the number is less than 25");
    }
    else if number >25 && number<50 {
        println!("the number is greater than 25 and less than 50");
    }
    else{
        println!("the number is greater than 50");
    }
}
