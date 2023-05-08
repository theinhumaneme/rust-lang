struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

impl Quadrilateral {
    fn square(size: u32) -> Self {
        return Self {
            width: size,
            height: size,
        };
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{0}", rect.area());

    let other_rect = Rectangle {
        width: 20,
        height: 20,
    };
    sq = Quadrilateral::square(3);
    println!("{}", rect.can_hold(&other_rect))
}
