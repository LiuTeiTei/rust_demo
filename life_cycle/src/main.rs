// `x` does not live long enough
/* fn main() {
    {
        let r;
        {
            let x = 5;
            r = &x;
        }
        println!("r: {}", r);
    }
} */

// error[E0106]: missing lifetime specifier
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
