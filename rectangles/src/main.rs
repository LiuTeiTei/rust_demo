#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 使用方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let width = 16;
    // let height = 20;

    // let rec = (16, 20);

    let rec = Rectangle {
        width: 16,
        height: 20,
    };
    println!(
        "the area of {} & {} is: {}",
        rec.width,
        rec.height,
        rec.area()
    );
    println!("Rectangle: {:?}", rec);
    println!("Rectangle: {:#?}", rec);

    let rec1 = Rectangle {
        width: 20,
        height: 16,
    };
    let rec2 = Rectangle {
        width: 15,
        height: 19,
    };
    println!("rec can hold rec1: {}", rec.can_hold(&rec1));
    println!("rec can hold rec2: {}", rec.can_hold(&rec2));

    let s = Rectangle::square(16);
    println!("Square: {:?}", s);
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// 使用元组类型来关联长方形的长和宽
// fn area(rec: (u32, u32)) -> u32 {
//     rec.0 * rec.1
// }

// 使用结构体来声明长方形的长和宽
// fn area(rec: &Rectangle) -> u32 {
//     rec.height * rec.width
// }
