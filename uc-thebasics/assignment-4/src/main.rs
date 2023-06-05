enum Shape{
    Triangle,
    Square,
    Pentagon,
    Hexagon,
    Heptagon,
    Octagon
}

impl Shape{

    fn return_corners(&self) -> i32{
        match &self{
            Shape::Triangle => 3,
            Shape::Square => 4,
            Shape::Pentagon => 5,
            Shape::Hexagon => 6,
            Shape::Heptagon => 7,
            Shape::Octagon => 8
        }
    }
}

fn main() {
    let shape = Shape::Hexagon;
    let number: i32 = shape.return_corners();
    println!("{}",number);
}
