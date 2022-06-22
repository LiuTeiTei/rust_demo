fn main() {
    println!("Hello, world!");
    another_function();
    print_value(16, '😂');

    let y = {
        let x = 3;
        // NOTE: there is no ';'
        x + 1
    };
    println!("The value of y is: {}", y);

    let x = plus_five(6);
    println!("The value of x is: {}", x)
}

// expected expression, found statement (`let`)
/* fn main() {
    let x = (let y = 6);
} */

fn another_function() {
    println!("Another function.");
}

fn print_value(x: i32, y: char) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_five(x: i32) -> i32 {
    // NOTE: there is no ';'
    x + 5
}

// error[E0308]: mismatched types, expected `i32`, found `()`, - help: consider removing this semicolon
/* fn plus_five(x: i32) -> i32 {
    // NOTE: there is no ';'
    x + 5;
} */
