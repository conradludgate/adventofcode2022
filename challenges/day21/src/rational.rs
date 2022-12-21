use core::fmt;
use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rational(i64, i64);

impl Default for Rational {
    fn default() -> Self {
        Self(0, 1)
    }
}
impl From<u16> for Rational {
    fn from(x: u16) -> Self {
        Self(x as i64, 1)
    }
}

impl Rational {
    pub const ZERO: Self = Self(0, 1);
    pub const ONE: Self = Self(1, 1);

    pub fn reduce(self) -> Self {
        let gcd = gcd(self.0.unsigned_abs(), self.1.unsigned_abs()) as i64 * self.1.signum();
        Self(self.0 / gcd, self.1 / gcd)
    }
    pub fn val(self) -> Option<i64> {
        let s = self.reduce();
        (s.1 == 1).then_some(s.0)
    }
    pub fn is_negative(self) -> bool {
        self.reduce().0.is_negative()
    }
}

fn gcd(mut x: u64, mut y: u64) -> u64 {
    loop {
        if y == 0 {
            return x;
        }
        match u64::cmp(&x, &y) {
            std::cmp::Ordering::Less => std::mem::swap(&mut x, &mut y),
            std::cmp::Ordering::Equal => break x,
            std::cmp::Ordering::Greater => (x, y) = (y, x % y),
        }
    }
}

impl ops::Mul for Rational {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1).reduce()
    }
}
impl ops::Div for Rational {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.1, self.1 * rhs.0).reduce()
    }
}
impl ops::Add for Rational {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.1 + rhs.0 * self.1, self.1 * rhs.1).reduce()
    }
}
impl ops::Sub for Rational {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.1 - rhs.0 * self.1, self.1 * rhs.1).reduce()
    }
}
impl ops::Neg for Rational {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, self.1)
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = self.reduce();
        if c.1 == 1 {
            c.0.fmt(f)
        } else {
            (c.0 as f64 / c.1 as f64).fmt(f)
        }
    }
}
