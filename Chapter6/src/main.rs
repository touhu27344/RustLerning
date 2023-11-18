/*
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
*/
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin:Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!.");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!",state);
            25
        },
    }
}

fn plus_one(x: Option<i32>)->Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),

    }
}

fn main() {
    /*
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    */

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _=> (),

    }
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn route(ip_type: IpAddr) { }
