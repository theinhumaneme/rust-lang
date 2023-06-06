fn main() {

    let mut car = Car{mpg: 32,top_speed:32, color:String::from("hello")};
    println!("{:?}",car);
    car.set_mpg(45);
    println!("{:?}",car);
}

pub trait SetMethods{
    fn set_top_speed(&mut self, top_speed:i32) -> i32;
    fn set_mpg(&mut self, mpg:i32) -> i32;
    fn set_color(&mut self, color:String) -> String;
}
#[derive(Debug)]
struct Car{
    mpg: i32,
    top_speed: i32,
    color: String,
}

struct Motorcycle{
    mpg: i32,
    top_speed: i32,
    color: String
}

impl SetMethods for Car{
    fn set_top_speed(&mut self, top_speed:i32) -> i32{
        self.top_speed = top_speed;
        return self.top_speed
    }
    fn set_mpg(&mut self, mpg:i32) -> i32{
        self.mpg = mpg;
        return self.mpg
    }
    fn set_color(&mut self, color:String) -> String{
        self.color = color;
        return self.color.clone()
    }
}

