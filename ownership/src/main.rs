/* variable */
/* fn main() {
    /* String */
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" word!");
    println!("{}", s);

    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    /* Move */
    let s1 = String::from("string move");
    let s2 = s1;
    // println!("s1: {}", s1); // error[E0382]: borrow of moved value: `s1`
    println!("only s2 validate: {}", s2);

    /* Clone */
    let s11 = String::from("string clone");
    let s22 = s11.clone();
    println!("s11: {}, s22: {}", s11, s22);
} */

/* function argument */
/* fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("s is invalidate: {}", s);  // error[E0382]: borrow of moved value: `s`

    let x = 5;
    makes_copy(x);
    println!("x still validate: {}", x);
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} */

/* function return */
/* fn main() {
    let s1 = gives_ownership();

    let s2: String = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("s2 is invalidate: {}", s2);  // error[E0382]: borrow of moved value: `s2`
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
} */

/* reference */
/* fn main() {
    let s = String::from("hello");
    let len = calculate_len(&s);
    println!("The length of {} is {}", s, len);

    let mut variable_s = String::from("hello");
    change(&mut variable_s);
    println!("The changed string is {}", variable_s);

    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // // error[E0499]: cannot borrow `s` as mutable more than once at a time
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{}", r1);
    }
    let r2 = &mut s;
    println!("{}", r2);

    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // let r3 = &mut s;
    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2); // 此位置之后 r1 和 r2 不再使用，因此可以再成功声明一个 r3
    let r3 = &mut s;
    println!("{}", r3);
}

fn calculate_len(some_string: &String) -> usize {
    some_string.len()
}

// fn changeError(some_string: &String) {
//     // error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
} */

/* Dangling References */
/* fn main() {
    // let reference_to_nothing = dangle();
    let reference_something = no_dangle();
    println!("{}", reference_something)
}

// error[E0106]: missing lifetime specifier
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
} */

/* Slice */
fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // s.clear();
    println!("{}", word);

    // let hello = &s[0..5];
    let hello = &s[..2];
    // let world = &s[6..11];
    // let world = &s[3..s.len()];
    let world = &s[3..];
    let whole = &s[..];
    println!("{}, {}, {}", hello, world, whole);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
