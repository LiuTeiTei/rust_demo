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

/**************************** assert_eq! & assert_ne! ****************************/
pub fn add_two(a: i32) -> i32 {
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
}
