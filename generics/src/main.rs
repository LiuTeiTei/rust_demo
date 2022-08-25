fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn y(&self) -> f32 {
        self.y
    }
}

struct MixPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixPoint<T, U> {
    fn mixup<V, W>(self, other: MixPoint<V, W>) -> MixPoint<T, W> {
        MixPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![1, 2, 35, 9, 444];
    let largest_number = largest(&number_list);
    println!("the largest number is {}", largest_number);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest_char = largest(&char_list);
    println!("The largest char is {}", largest_char);

    let integer = Point { x: 10, y: 20 };
    let float = Point { x: 10.0, y: 20.0 };
    let integer_float = MixPoint { x: 10, y: 20.0 };
    println!("p.x={}", integer.x());
    println!("p.y={}", float.y());
    let p1 = MixPoint { x: 5, y: 10.4 };
    let p2 = MixPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
