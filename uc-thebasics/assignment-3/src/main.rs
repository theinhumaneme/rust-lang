#[derive(Debug)]
struct Car{
    mpg: u32,
    color: String,
    top_speed: u32
}


impl Car {
    fn set_mpg(&mut self, mpg: u32){
        self.mpg = mpg;
    }
    fn set_color(&mut self, color: String){
        self.color = color;
    }
    fn set_top_speed(&mut self, top_speed: u32){
        self.top_speed = top_speed;
    }
}


fn main() {
    let mut car = Car{mpg:32,color:String::from("red"),top_speed:45};
    println!("{:?}",car);
    car.set_color(String::from("blue"));
    car.set_top_speed(50);
    car.set_mpg(100);
    println!("{:?}",car)
    
}
