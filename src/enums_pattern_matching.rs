#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);
    println!("{}", value_in_cents(Coin::Quarter));
    println!("1 + 1 = {}", plus_one(Some(1)));

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
            println!("What's a Quarter ? I don't know 😭");
            25
        }
    }
}
fn plus_one(x: Option<u32>) -> u32 {
    match x {
        None => 1,
        Some(i) => i + 1,
    }
}