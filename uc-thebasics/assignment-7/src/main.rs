use std::rc::Rc;

fn main() {
    let num = 10;
    let heap_num = Box::new(10);
    let mul_num = num * *heap_num;
    println!("{:?}", mul_num);

    let string: String = String::from("hello world");
    let rc: Rc<String> = Rc::new(string);
    println!("{:?}", Rc::strong_count(&rc));
    {
        let rc2 = Rc::clone(&rc);
        println!("{:?}", Rc::strong_count(&rc2))
    }
    println!("{:?}", Rc::strong_count(&rc));
}
