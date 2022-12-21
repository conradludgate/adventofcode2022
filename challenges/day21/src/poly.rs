use core::fmt;
use std::ops;

use tinyvec::{array_vec, TinyVec};

use crate::rational::Rational;

#[derive(Debug, PartialEq, Clone)]
pub struct Poly {
    mantissa: TinyVec<[Rational; 2]>,
    exponent: isize,
}

impl ops::Mul for Poly {
    type Output = Self;
    fn mul(mut self, mut rhs: Self) -> Self::Output {
        let exponent = self.exponent + rhs.exponent;
        let mantissa = if let [y] = rhs.mantissa.as_slice() {
            self.mantissa.iter_mut().for_each(|x| *x = *x * *y);
            self.mantissa
        } else if let [x] = self.mantissa.as_slice() {
            rhs.mantissa.iter_mut().for_each(|y| *y = *x * *y);
            rhs.mantissa
        } else {
            let mut mantissa = TinyVec::new();
            mantissa.resize(self.mantissa.len() + rhs.mantissa.len() - 1, Rational::ZERO);

            for (i, l) in self.mantissa.into_iter().enumerate() {
                for (j, r) in rhs.mantissa.iter().copied().enumerate() {
                    mantissa[i + j] = mantissa[i + j] + l * r;
                }
            }
            mantissa
        };
        Poly { exponent, mantissa }
    }
}
impl ops::Div for Poly {
    type Output = Self;
    fn div(mut self, mut rhs: Self) -> Self::Output {
        rhs.reduce();
        self.exponent -= rhs.exponent;
        let [r] = rhs.mantissa.as_slice() else { todo!("higher order division") };
        self.mantissa.iter_mut().for_each(|x| *x = *x / *r);
        self
    }
}
impl ops::Add for Poly {
    type Output = Self;
    fn add(mut self, mut rhs: Self) -> Self::Output {
        let diff = self.partial_degree() - rhs.partial_degree();
        let diff = if diff < 0 {
            std::mem::swap(&mut self, &mut rhs);
            -diff as usize
        } else {
            diff as usize
        };
        self.normalise_with(&mut rhs);

        for (l, r) in self.mantissa[diff..].iter_mut().zip(rhs.mantissa) {
            *l = *l + r;
        }

        self
    }
}
impl ops::Sub for Poly {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}
impl ops::Neg for Poly {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.mantissa.iter_mut().for_each(|x| *x = -*x);
        self
    }
}

impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = self.partial_degree();
        if d == 0 {
            return write!(
                f,
                "{}",
                self.mantissa.last().copied().unwrap_or(Rational::ONE)
            );
        }

        let mut first = true;
        for (i, mut c) in self.mantissa.iter().copied().enumerate() {
            let power = d - i as isize;
            if c == Rational::ZERO {
                continue;
            }

            if c.is_negative() && first {
                write!(f, "-")?;
                c = -c;
            } else if c.is_negative() && !first {
                write!(f, " - ")?;
                c = -c;
            } else if !first {
                write!(f, " + ")?;
            }
            first = false;

            if c != Rational::ONE || power == 0 {
                c.fmt(f)?
            }

            if power != 0 {
                write!(f, "x")?;
                if power != 1 {
                    write!(f, "^{power}")?;
                }
            }
        }

        Ok(())
    }
}

impl From<Rational> for Poly {
    fn from(value: Rational) -> Self {
        Self {
            mantissa: TinyVec::Inline(array_vec!(_ => value)),
            exponent: 0,
        }
    }
}

impl Poly {
    pub fn x() -> Self {
        Self {
            mantissa: TinyVec::Inline(array_vec!(_ => Rational::ONE)),
            exponent: 1,
        }
    }

    fn partial_degree(&self) -> isize {
        (self.mantissa.len() as isize) + self.exponent - 1
    }
    fn normalise_with(&mut self, other: &mut Self) {
        self.reduce();
        other.reduce();
        match self.exponent.cmp(&other.exponent) {
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Less => {
                let len = other.mantissa.len() + (other.exponent - self.exponent) as usize;
                other.mantissa.resize(len, Rational::ZERO);
                other.exponent = self.exponent;
            }
            std::cmp::Ordering::Greater => {
                let len = self.mantissa.len() + (self.exponent - other.exponent) as usize;
                self.mantissa.resize(len, Rational::ZERO);
                self.exponent = other.exponent;
            }
        }
    }

    /// Reduces the polynomial to have the least amount of trailing and leading zeros
    fn reduce(&mut self) {
        // trim trailing 0s (increasing the exp)
        let mut len = self.mantissa.len();
        for c in self.mantissa.iter().rev() {
            if *c == Rational::ZERO {
                len -= 1;
                self.exponent += 1;
                continue;
            }
            break;
        }
        self.mantissa.truncate(len);

        // trim leading 0s
        let mut leading = 0;
        for c in self.mantissa.iter() {
            if *c == Rational::ZERO {
                leading += 1;
                continue;
            }
            break;
        }
        self.mantissa.drain(..leading);
    }

    pub fn eval(self, x: Rational) -> Rational {
        let mut xi = Rational::ONE;
        for _ in 0..self.exponent {
            xi = xi * x;
        }
        for _ in self.exponent..0 {
            xi = xi / x;
        }

        let mut output = Rational::ZERO;
        for coef in self.mantissa.into_iter().rev() {
            output = output + (coef * xi);
            xi = xi * x;
        }
        output
    }

    // solve self == 0
    // (currently only works for linear polynomials)
    pub fn solve(self) -> Rational {
        // (ax + b)*x^exp = 0
        let [a, b] = self.mantissa.as_slice() else { todo!("higher order polynomials") };
        // ax + b = 0
        if self.exponent != 0 {
            todo!("higher order polynomials")
        }
        -*b / *a
    }
}
