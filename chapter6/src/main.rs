fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    //
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };


    // let home = IpAddr::V4(String::from("127.0.0.1"))

    // let home = IpAddr::V4(127, 0, 0, 1);
    //
    // let loopback = IpAddr::V6(String::from("::1"));

    // let m = Message::Write(String::from("hello"));
    // m.call();


    // Option
    // let some_number = Some(5);
    // let some_char = Some('e');
    //
    // let absent_number: Option<i32> = None;


    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    //
    // let sum = x + y.unwrap();
    //
    // println!("{}", sum)

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // констркция с if let, которая делает тоже что и конструкция с match
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     //V4(String),
//     V6(String),
// }

// struct Ipv4Addr {
//     // --snip--
// }
//
// struct Ipv6Addr {
//     // --snip--
// }
//
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// структуры, которые соответствуют перечеслению выше
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

// impl Message {
//     fn call(&self) {
//         // method body would be defined here
//     }
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}