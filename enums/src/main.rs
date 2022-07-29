#[derive(Debug)]
enum IpAddrKind {
    v4,
    v6,
}

#[derive(Debug)]
enum IpAddrEnum {
    V4(String),
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        println!("Message call self:{:#?}", self);
    }
}

fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;
    route(&four);
    println!("IpAddrKind:{:?},{:?}", four, six);

    let home1 = IpAddr {
        kind: four,
        address: String::from("127.0.0.1"),
    };
    let loopback1 = IpAddr {
        kind: six,
        address: String::from("::1"),
    };
    println!("IpAddr:{:#?},{:#?}", home1, loopback1);

    let home2 = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddrEnum::V6(String::from("::1"));
    println!("IpAddrEnum:{:#?},{:#?}", home2, loopback2);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let absent_number: Option<i32> = None;
    plus_one(some_number);
    plus_one(absent_number);
    println!("Option some: {:#?} none: {:#?}", some_number, absent_number);

    let x: i32 = 8;
    let y: Option<i32> = Some(8);
    // error[E0277]: cannot add `Option<i8>` to `i8`
    // let sum = x + y;

    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(ChinaProvince::BeiJing));

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // 行为一致
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn route(ip_kind: &IpAddrKind) {
    println!("ip_kind:{:?}", ip_kind);
}

enum ChinaProvince {
    GuangZhou,
    BeiJing,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(ChinaProvince),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(province) => {
            let status = province;
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}
