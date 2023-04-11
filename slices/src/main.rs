fn main() {
    let string = String::from("Kalyan Mudumby");
    let word = first_word(&string);
    println!("{word}")

}
fn first_word(string: &String) -> &str
{
    let string_bytes = string.as_bytes();
    for (i, &item) in string_bytes.iter().enumerate(){
        if item == b' '{
            return &string[0..i];
        }
    }
    return &string[..]
}
