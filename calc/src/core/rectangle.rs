#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        if width == 0 {
            panic!(
                "width は 0 より大きい値でなければなりません: width={}",
                width
            );
        }
        if height == 0 {
            panic!(
                "height は 0 より大きい値でなければなりません: height={}",
                height
            );
        }

        Rectangle { width, height }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_2_by_3_is_ok() {
        // given
        // when
        let rectangle = Rectangle::new(2, 3);

        // then
        assert_eq!(rectangle.width, 2);
        assert_eq!(rectangle.height, 3);
    }

    #[test]
    #[should_panic(expected = "width は 0 より大きい値でなければなりません: width=0")]
    fn new_0_by_3_occur_panic() {
        // given
        Rectangle::new(0, 3);
    }

    #[test]
    #[should_panic(expected = "height は 0 より大きい値でなければなりません: height=0")]
    fn new_2_by_0_occur_panic() {
        // given
        Rectangle::new(2, 0);
    }

    #[test]
    fn area_2_by_3_equals_6() {
        // given
        let rectangle = Rectangle::new(2, 3);

        // when
        let result = rectangle.area();

        // then
        assert_eq!(result, 6);
    }

    #[test]
    fn is_square_2_by_2_is_true() {
        // given
        let rectangle = Rectangle::new(2, 2);

        // when
        let result = rectangle.is_square();

        // then
        assert!(result);
    }

    #[test]
    fn is_square_2_by_3_is_false() {
        // given
        let rectangle = Rectangle::new(2, 3);

        // when
        let result = rectangle.is_square();

        // then
        assert!(!result);
    }

    #[test]
    fn larger_can_hold_smaller() {
        // given
        let rectangle1 = Rectangle::new(2, 2);
        let rectangle2 = Rectangle::new(1, 1);

        // when
        let result = rectangle1.can_hold(&rectangle2);

        // then
        assert!(result);
    }

    #[test]
    fn cannot_hold_same_size_rectangle() {
        // given
        let rectangle1 = Rectangle::new(2, 2);
        let rectangle2 = Rectangle::new(2, 2);

        // when
        let result = rectangle1.can_hold(&rectangle2);

        // then
        assert!(!result);
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        // given
        let rectangle1 = Rectangle::new(2, 2);
        let rectangle2 = Rectangle::new(3, 3);

        // when
        let result = rectangle1.can_hold(&rectangle2);

        // then
        assert!(!result);
    }
}
