#[allow(dead_code)]
pub(crate) trait IsDivisible {
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
    fn odd_ceil(&self) -> Self;
    fn odd_floor(&self) -> Self;
}

impl IsDivisible for usize {
    fn is_even(&self) -> bool {
        self.is_multiple_of(2)
    }

    fn is_odd(&self) -> bool {
        !self.is_even()
    }

    fn odd_ceil(&self) -> Self {
        if self.is_odd() { *self } else { *self + 1 }
    }

    fn odd_floor(&self) -> Self {
        match self {
            0 => 1,
            _ if self.is_even() => *self - 1,
            _ => *self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IsDivisible;

    #[test]
    fn parity_checks_work() {
        assert!(2usize.is_even());
        assert!(!2usize.is_odd());
        assert!(!3usize.is_even());
        assert!(3usize.is_odd());
    }

    #[test]
    fn odd_adjustments_work() {
        assert_eq!(5usize.odd_ceil(), 5);
        assert_eq!(6usize.odd_ceil(), 7);
        assert_eq!(0usize.odd_floor(), 1);
        assert_eq!(8usize.odd_floor(), 7);
        assert_eq!(9usize.odd_floor(), 9);
    }
}
