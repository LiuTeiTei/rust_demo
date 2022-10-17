/**************************** default ****************************/
/* #[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn error() {
        panic!("Make this test fail");
    }
} */

/**************************** assert! ****************************/
/* struct Rectangle {
    width: u32,
    hight: u32,
}

impl Rectangle {
    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.hight > other.hight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_small() {
        let lager = Rectangle { width: 8, hight: 7 };
        let small = Rectangle { width: 5, hight: 1 };
        assert!(lager.can_hold(small))
    }

    #[test]
    fn larger_cannot_hold_small() {
        let lager = Rectangle { width: 8, hight: 7 };
        let small = Rectangle { width: 5, hight: 1 };
        assert!(!small.can_hold(lager))
    }
} */

/**************************** should_panic ****************************/
/* pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use crate::add_two;

    #[test]
    fn it_add_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(4, add_two(2));
    }
} */

/**************************** assert_eq! & assert_ne! ****************************/
/* pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}",
                value
            )
        }

        Guess { value }
    }
}

#[cfg(test)]
mod test {
    use crate::Guess;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(0);
    }
} */

/**************************** Result<T, E> ****************************/
/* #[cfg(test)]
mod test {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 3 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
} */

/**************************** ignore ****************************/
/* #[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // 需要运行一个小时的代码
    }
} */

/**************************** test private ****************************/
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
