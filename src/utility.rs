#[allow(dead_code)]
pub(crate) trait IsEvenOdd {
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
    fn even_ceil(&self) -> Self;
    fn even_floor(&self) -> Self;
    fn odd_ceil(&self) -> Self;
    fn odd_floor(&self) -> Self;
}

impl IsEvenOdd for usize {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }

    fn is_odd(&self) -> bool {
        self % 2 != 0
    }

    fn even_ceil(&self) -> Self {
        if self.is_even() { *self } else { *self + 1 }
    }

    fn even_floor(&self) -> Self {
        if self.is_even() { *self } else { *self - 1 }
    }

    fn odd_ceil(&self) -> Self {
        if self.is_odd() { *self } else { *self + 1 }
    }

    fn odd_floor(&self) -> Self {
        match self {
            0 => 1,
            _ if self.is_odd() => *self,
            _ if self.is_even() => *self - 1,
            _ => *self,
        }
    }
}
