fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(1);

    let third: &i32 = &v2[2];
    println!("The third element is {}", third);
    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let mut v3 = vec![1];
    let first = &v3[0];
    // error[E0502]: cannot borrow `v3` as mutable because it is also borrowed as immutable
    // v3.push(2);
    println!("The first element is {}", first);

    let mut v4 = vec![0, 1, 2];
    for i in &mut v4 {
        *i *= 10;
    }
    for i in &v4 {
        println!("{}", i)
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(16),
        SpreadsheetCell::Text(String::from("black")),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("test")),
    ];

    let mut s1 = String::new();

    let data = "hello world";
    let s2 = data.to_string();
    let s3 = "hello world".to_string();
    let s4 = String::from("hello world");

    s1.push_str(data);
    s1.push_str(&s4);
    println!("{},{},{}", s1, data, s4);

    let s5 = s2 + "_" + &s4 + "_" + data;
    // error[E0382]: borrow of moved value: `s2`
    // println!("{},{},{}", s1, data, s2);

    let s6 = format!("{}_{}_{}", s3, s4, data);

    let len = String::from("三").len();
    println!("{}", len); // 3

    let word = "नमस्ते";
    let w1 = &word[0..2];
    println!("{}", w1); // न
}
