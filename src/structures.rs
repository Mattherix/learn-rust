#[derive(Clone, Debug)]
struct Color(u32, u32, u32);
#[derive(Clone, Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
    color: Color,
    rectangle: Rectangle
}
fn main() {
    let mut alice = User {
        username: String::from("Alice"),
        email: String::from("alice@mail.com"),
        sign_in_count: 0,
        active: true,
        color: Color(0, 0, 0),
        rectangle: Rectangle {
            width: 2,
            height: 5
        }
    };
    alice.email = String::from("alice@mail.net");
    
    let bob = User {
        username: String::from("Bob"),
        email: String::from("bob@mail.com"),
        // ..alice <- can't be borrow
        sign_in_count: 0,
        active: true,
        color: alice.color.clone(),
        rectangle: alice.rectangle.clone()

    };
    alice.rectangle.width = 1;
    alice.rectangle.height = 2;
    println!("{:?}", bob.rectangle.area());
    println!("{:?}", bob);
    println!("{:?}", &alice);
    println!("{:?}", bob);
    println!("Can bob rectangle hold alice rectangle ? -> {}", bob.rectangle.can_hold(&alice.rectangle));
    println!("Can alice rectangle hold bob rectangle ? -> {}", alice.rectangle.can_hold(&bob.rectangle));
    println!("Can bob rectangle hold a 2 by 2 square ? -> {}", bob.rectangle.can_hold(&Rectangle::square(2)));
}
impl Rectangle {
    fn square(side: u32) -> Rectangle{
        Rectangle {
            width: side,
            height: side
        }
    }
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self, rectangle: &Rectangle) -> bool{
        self.width >= rectangle.width && self.height >= rectangle.height
    }
}

