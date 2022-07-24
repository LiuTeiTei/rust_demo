#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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
        area(&rec)
    );
    println!("Rectangle: {:?}", rec);
    println!("Rectangle: {:#?}", rec);
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// 使用元组类型来关联长方形的长和宽
// fn area(rec: (u32, u32)) -> u32 {
//     rec.0 * rec.1
// }

// 使用结构体来声明长方形的长和宽
fn area(rec: &Rectangle) -> u32 {
    rec.height * rec.width
}
