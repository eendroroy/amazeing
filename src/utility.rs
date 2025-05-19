#[allow(dead_code)]
pub(crate) trait IsDivisible {
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
    fn is_quadruple(&self) -> bool;
    fn even_ceil(&self) -> Self;
    fn even_floor(&self) -> Self;
    fn odd_ceil(&self) -> Self;
    fn odd_floor(&self) -> Self;
    fn quadruple_ceil(&self) -> Self;
    fn quadruple_floor(&self) -> Self;
}

impl IsDivisible for usize {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }

    fn is_odd(&self) -> bool {
        !self.is_even()
    }

    fn is_quadruple(&self) -> bool {
        self % 4 == 0
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
            _ if self.is_even() => *self - 1,
            _ => *self,
        }
    }

    fn quadruple_ceil(&self) -> Self {
        if self.is_quadruple() { *self } else { (1 + *self / 4) * 4 }
    }

    fn quadruple_floor(&self) -> Self {
        if self.is_quadruple() { *self } else { 4 * *self / 4 }
    }
}
