use std::fs::File;
use std::io::ErrorKind;

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
}
