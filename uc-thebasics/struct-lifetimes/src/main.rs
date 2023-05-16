#[derive(Debug)]
struct User {
    username:String,
    active: bool,
    logins: u32
}
 #[derive(Debug)]
 struct Quad{
     width: i32,
     height: i32
}

impl Quad {

    fn perimeter(&self) -> i32 {
      2*(self.width+self.height) 
    }

    fn area(&self) -> i32{
        self.width * self.height
    }
    // add code here
}
fn build_user(username: &String) -> User{
    User{
        username: username.to_string(),
        active: true,
        logins: 1
    }
}

fn build_quad(width: i32, height: i32) -> Quad{
   Quad { width, height} 
}

fn main() {

    let user: User = build_user(&String::from("Kalyan"));
    print!("{:?}",user.username);
    let quad = build_quad(32,32);
    print!("{:?}",quad.perimeter());
    print!("{:?}",quad.area());
}
