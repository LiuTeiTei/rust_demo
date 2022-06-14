// error[E0384]: cannot assign twice to immutable variable `x`: cannot assign twice to immutable variable
/* fn main() {
    let x = 5;
    x = 6;
} */

// error[E0308]: mismatched types: expected integer, found `char`
/* fn main() {
    let mut x = 5;
    x = '6';
} */

// const MAX_POINT: u32 = 99_999;

// fn main() {
//     const MIN_POINT: u32 = 1_000;
//     println!("the value of MAX_POINT is: {}", MAX_POINT);
//     println!("the value of MIN_POINT is: {}", MIN_POINT);
// }

fn main() {
    let spaces = "    ";
    let spaces = spaces.len();
    println!("the length of spaces is:{}", spaces);

    let x = 10;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
