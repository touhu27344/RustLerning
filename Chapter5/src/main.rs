//
//構造体
//

//構造体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//タプル構造体
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    /*
    let mut user1  = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}",user1.email);

    user1.email = String::from("anothermail@example.com");
    print!("{}",user1.email);

    let user2 = User{
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    */

    //
    //使用例
    //

    let rect2 = Rectangle { width: 30, height: 50 };

    println!("rect2 is {:?}", rect2);
    println!("The area of the rectangle is {} square pixels.",
            rect2.area()
        );


}

fn build_user (email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
