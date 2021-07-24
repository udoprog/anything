use num::{BigInt, BigRational, Signed, ToPrimitive, Zero};
use std::fmt;
use std::fmt::Write;
use std::mem;

/// Perform formatting of a big rational.
pub struct Display<'a> {
    rational: &'a BigRational,
    limit: usize,
    exponent_limit: i32,
    cap: bool,
}

impl<'a> Display<'a> {
    pub(crate) fn new(
        rational: &'a BigRational,
        limit: usize,
        exponent_limit: i32,
        cap: bool,
    ) -> Self {
        Self {
            rational,
            limit,
            exponent_limit,
            cap,
        }
    }
}

impl fmt::Display for Display<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let neg = self.rational.is_negative();
        let mut rem = self.rational.numer().abs();
        let den = self.rational.denom().abs();

        let div = &rem / &den;
        rem -= &den * &div;

        if !div.is_zero() || rem.is_zero() {
            if neg {
                f.write_char('-')?;
            }

            div.fmt(f)?;

            if rem.is_zero() {
                return Ok(());
            }

            if self.limit > 0 {
                f.write_char('.')?;

                for d in emit(&mut rem, &den).take(self.limit) {
                    d.fmt(f)?;
                }
            }

            if !rem.is_zero() && self.cap {
                f.write_char('…')?;
            }

            return Ok(());
        }

        let mut exp = -1i32;
        let mut init = true;
        let mut dot = true;
        let mut takes_exp = true;
        let mut n = self.limit;

        for d in emit(&mut rem, &den) {
            if n == 0 {
                break;
            }

            if d.is_zero() && takes_exp {
                exp -= 1;
                continue;
            }

            takes_exp = false;
            n -= 1;

            if mem::take(&mut init) {
                if neg {
                    f.write_char('-')?;
                }

                if exp <= self.exponent_limit {
                    d.fmt(f)?;
                    continue;
                }

                f.write_str("0.")?;

                for _ in exp..-1 {
                    f.write_char('0')?;
                }

                d.fmt(f)?;
                exp = 0;
                dot = false;
            } else {
                if mem::take(&mut dot) {
                    f.write_char('.')?;
                }

                d.fmt(f)?;
            }
        }

        if !rem.is_zero() && self.cap {
            f.write_char('…')?;
        }

        if exp != 0 {
            write!(f, "e{}", exp)?;
        }

        Ok(())
    }
}

/// Internal helper to keep diving a value and emitting its values.
///
/// Each emitted value is guaranteed to be smaller than 10.
fn emit<'a>(rem: &'a mut BigInt, den: &'a BigInt) -> impl Iterator<Item = u8> + 'a {
    std::iter::from_fn(move || {
        if rem.is_zero() {
            return None;
        }

        *rem *= 10u32;
        let div = &*rem / den;
        *rem -= den * &div;

        let div = div.to_u8()?;
        debug_assert!(div < 10u8);
        Some(div)
    })
}
