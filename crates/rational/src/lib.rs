use num::traits::Pow;
use num::{BigInt, BigRational, One, ToPrimitive, Zero};
use serde::{de, ser};
use std::str::FromStr;
use std::{fmt, ops};
use thiserror::Error;

mod display;
pub use self::display::Display;

#[cfg(test)]
mod tests;

/// A rational number.
#[derive(Clone, PartialEq, Eq)]
pub struct Rational {
    rational: BigRational,
}

impl Rational {
    /// Construct a new rational number.
    pub fn new<N, D>(numer: N, denom: D) -> Self
    where
        N: Into<BigInt>,
        D: Into<BigInt>,
    {
        Self {
            rational: num::BigRational::new(numer.into(), denom.into()),
        }
    }

    /// Test if this is an integer.
    pub fn is_integer(&self) -> bool {
        self.rational.is_integer()
    }

    /// Gets an immutable reference to the numerator.
    pub fn numer(&self) -> &BigInt {
        self.rational.numer()
    }

    /// Gets an immutable reference to the denominator.
    pub fn denom(&self) -> &BigInt {
        self.rational.denom()
    }

    /// Construct from a 64-bit float.
    pub fn from_f64(value: f64) -> Self {
        let rational =
            BigRational::from_float(value).unwrap_or_else(|| BigRational::new(1.into(), 1.into()));

        Self { rational }
    }

    /// Returns the reciprocal.
    ///
    /// # Panics
    ///
    /// Panics if the `Ratio` is zero.
    pub fn recip(self) -> Self {
        Self {
            rational: self.rational.recip(),
        }
    }

    /// Format this rational number.
    pub fn display(&self, limit: usize, exponent_limit: usize, cap: bool) -> Display<'_> {
        Display::new(&self.rational, limit, exponent_limit, cap)
    }

    /// Raises the `Ratio` to the power of an exponent.
    #[inline]
    pub fn pow(&self, expon: i32) -> Rational
    where
        for<'a> &'a BigInt: Pow<u32, Output = BigInt>,
    {
        Self {
            rational: Pow::pow(&self.rational, expon),
        }
    }

    /// Round the current rational to it's closest whole number.
    pub fn round(&self) -> Rational {
        Self {
            rational: self.rational.round(),
        }
    }

    /// Round this rational to its floor.
    pub fn floor(&self) -> Rational {
        let rational = if self.rational.denom().is_one() {
            self.rational.clone()
        } else {
            self.rational.trunc()
        };

        Self { rational }
    }

    /// Round this rational to its ceilting.
    pub fn ceil(&self) -> Rational {
        let rational = if self.rational.denom().is_one() {
            self.rational.clone()
        } else {
            (self.rational.clone() + BigRational::one()).trunc()
        };

        Self { rational }
    }
}

impl ops::Add<Rational> for Rational {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        Self {
            rational: self.rational + rhs.rational,
        }
    }
}

impl ops::Sub<Rational> for Rational {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        Self {
            rational: self.rational + rhs.rational,
        }
    }
}

impl ops::Div<Rational> for Rational {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        Self {
            rational: self.rational / rhs.rational,
        }
    }
}

impl ops::Div<&Rational> for Rational {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        Self {
            rational: self.rational / &rhs.rational,
        }
    }
}

impl ops::Div<&Rational> for &Rational {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        Rational {
            rational: &self.rational / &rhs.rational,
        }
    }
}

impl ops::Mul<Rational> for Rational {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        Self {
            rational: self.rational * rhs.rational,
        }
    }
}

impl ops::Mul<&Rational> for Rational {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        Self {
            rational: self.rational * &rhs.rational,
        }
    }
}

impl ops::Mul<&Rational> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        Rational {
            rational: &self.rational * &rhs.rational,
        }
    }
}

impl ops::AddAssign<Rational> for Rational {
    fn add_assign(&mut self, rhs: Rational) {
        self.rational += rhs.rational;
    }
}

impl ops::AddAssign<&Rational> for Rational {
    fn add_assign(&mut self, rhs: &Rational) {
        self.rational += &rhs.rational;
    }
}

impl ops::SubAssign<Rational> for Rational {
    fn sub_assign(&mut self, rhs: Rational) {
        self.rational -= rhs.rational;
    }
}

impl ops::SubAssign<&Rational> for Rational {
    fn sub_assign(&mut self, rhs: &Rational) {
        self.rational -= &rhs.rational;
    }
}

impl ops::MulAssign<Rational> for Rational {
    fn mul_assign(&mut self, rhs: Rational) {
        self.rational *= rhs.rational;
    }
}

impl ops::MulAssign<&Rational> for Rational {
    fn mul_assign(&mut self, rhs: &Rational) {
        self.rational *= &rhs.rational;
    }
}

impl ops::DivAssign<Rational> for Rational {
    fn div_assign(&mut self, rhs: Rational) {
        self.rational /= rhs.rational;
    }
}

impl ops::DivAssign<&Rational> for Rational {
    fn div_assign(&mut self, rhs: &Rational) {
        self.rational /= &rhs.rational;
    }
}

impl One for Rational {
    fn one() -> Self {
        Self {
            rational: One::one(),
        }
    }

    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        self.rational.is_one()
    }
}

impl Zero for Rational {
    fn zero() -> Self {
        Self {
            rational: Zero::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.rational.is_zero()
    }
}

impl ToPrimitive for Rational {
    fn to_i8(&self) -> Option<i8> {
        self.rational.to_i8()
    }

    fn to_i16(&self) -> Option<i16> {
        self.rational.to_i16()
    }

    fn to_i128(&self) -> Option<i128> {
        self.rational.to_i128()
    }

    fn to_usize(&self) -> Option<usize> {
        self.rational.to_usize()
    }

    fn to_u8(&self) -> Option<u8> {
        self.rational.to_u8()
    }

    fn to_isize(&self) -> Option<isize> {
        self.rational.to_isize()
    }

    fn to_u16(&self) -> Option<u16> {
        self.rational.to_u16()
    }

    fn to_u32(&self) -> Option<u32> {
        self.rational.to_u32()
    }

    fn to_u64(&self) -> Option<u64> {
        self.rational.to_u64()
    }

    fn to_u128(&self) -> Option<u128> {
        self.rational.to_u128()
    }

    fn to_f32(&self) -> Option<f32> {
        self.rational.to_f32()
    }

    fn to_f64(&self) -> Option<f64> {
        self.rational.to_f64()
    }

    fn to_i32(&self) -> Option<i32> {
        self.rational.to_i32()
    }

    fn to_i64(&self) -> Option<i64> {
        self.rational.to_i64()
    }
}

/// Failed to parse a rational number.
#[derive(Debug, Error)]
#[error("illegal numeric value")]
pub struct ParseRationalError(());

impl FromStr for Rational {
    type Err = ParseRationalError;

    fn from_str(number: &str) -> Result<Self, Self::Err> {
        let mut dot = false;
        let mut init = false;
        let mut dots = 0u32;

        let mut out = BigRational::new(0u32.into(), 1u32.into());
        let ten = &BigInt::from(10u32);

        let mut it = number.bytes().peekable();

        let neg = if let Some(b'-' | b'+') = it.peek() {
            matches!(it.next(), Some(b'-'))
        } else {
            false
        };

        while let Some(b) = it.next() {
            match b {
                // Ignore leading zeros.
                b'0' if !init => {
                    continue;
                }
                b'0'..=b'9' => {
                    init = true;

                    let c = (b - b'0') as u32;
                    out *= ten;
                    out += BigInt::from(c);

                    if dot {
                        dots = dots.checked_add(1).ok_or(ParseRationalError(()))?;
                    }
                }
                b'.' if !dot => {
                    init = true;
                    dot = true;
                }
                b'e' | b'E' => {
                    let neg = if let Some(b'-' | b'+') = it.peek() {
                        matches!(it.next(), Some(b'-'))
                    } else {
                        false
                    };

                    let mut exp = 0u32;
                    let mut init = false;

                    for b in it {
                        match b {
                            // Ignore leading zeros.
                            b'0' if !init => {
                                continue;
                            }
                            b'0'..=b'9' => {
                                init = true;
                                let n = (b - b'0') as u32;

                                exp = match exp.checked_mul(10).and_then(|exp| exp.checked_add(n)) {
                                    Some(exp) => exp,
                                    None => return Err(ParseRationalError(())),
                                };
                            }
                            _ => {
                                return Err(ParseRationalError(()));
                            }
                        }
                    }

                    if neg {
                        out /= ten.pow(exp);
                    } else {
                        out *= ten.pow(exp);
                    }

                    break;
                }
                _ => {
                    return Err(ParseRationalError(()));
                }
            }
        }

        out /= ten.pow(dots);
        let out = if neg { -out } else { out };
        Ok(Rational { rational: out })
    }
}

impl<'de> de::Deserialize<'de> for Rational {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let rational = BigRational::deserialize(deserializer)?;

        Ok(Self { rational })
    }
}

impl ser::Serialize for Rational {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.rational.serialize(serializer)
    }
}

impl fmt::Debug for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display(6, 8, true))
    }
}
