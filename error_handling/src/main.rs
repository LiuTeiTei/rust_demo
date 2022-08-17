use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    // 原始写法
    // let file = File::open("hello.txt");

    // let mut file = match file {
    //     Ok(f) => f,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match file.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => return Err(e),
    // }

    // `?` 简写
    // let mut file = File::open("hello.txt")?;
    // let mut s = String::new();
    // file.read_to_string(&mut s)?;
    // Ok(s)

    // 链式方法调用
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s);
    Ok(s)
}

fn main() {
    // file 的类型是 std::result::Result<std::fs::File, std::io::Error>
    let file = File::open("hello.txt");

    // let file = match file {
    //     Ok(f) => f,
    //     Err(e) => panic!("Error opening the file: {:?}", e),
    // };

    let file = match file {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(ce) => panic!("Error creating the file: {:?}", ce),
            },
            other_error => panic!("Error opening the file: {:?}", other_error),
        },
    };

    let file = File::open("hello.txt").unwrap();
    let file = File::open("hello.txt").expect("无法打开文件");

    let result = read_username_from_file();
    println!("{:?}", result);
}
