use num::{BigInt, BigRational, Signed, ToPrimitive, Zero};
use std::fmt;
use std::fmt::Write;
use std::mem;

/// A display specification for a rational number.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct DisplaySpec {
    pub limit: usize,
    pub exponent_limit: usize,
    pub cap: bool,
}

impl Default for DisplaySpec {
    fn default() -> Self {
        Self {
            limit: 6,
            exponent_limit: 8,
            cap: true,
        }
    }
}

/// Perform formatting of a big rational.
pub struct Display<'a> {
    rational: &'a BigRational,
    spec: &'a DisplaySpec,
}

impl<'a> Display<'a> {
    pub(crate) fn new(rational: &'a BigRational, spec: &'a DisplaySpec) -> Self {
        Self { rational, spec }
    }

    /// Format a big number.
    fn format_big(
        &self,
        f: &mut fmt::Formatter<'_>,
        neg: bool,
        mut rem: BigInt,
        div: BigInt,
        den: &BigInt,
    ) -> fmt::Result {
        if neg {
            f.write_char('-')?;
        }

        let string = div.to_string();
        let mut it = string.chars().peekable();

        if let Some(d) = it.next() {
            fmt::Display::fmt(&d, f)?;
        }

        if it.peek().is_some() {
            f.write_char('.')?;
        }

        let mut used = 0;

        for d in (&mut it).take(self.spec.limit) {
            fmt::Display::fmt(&d, f)?;
            used += 1;
        }

        let dot = if it.peek().is_some() {
            true
        } else {
            let remaining = self.spec.limit - used;

            if remaining > 0 {
                let mut it = emit(&mut rem, den);

                for d in (&mut it).take(remaining) {
                    fmt::Display::fmt(&d, f)?;
                }

                it.next().is_some()
            } else {
                false
            }
        };

        if dot {
            f.write_char('…')?;
        }

        let exp = it.count() + used;

        if exp > 0 {
            f.write_char('e')?;
            fmt::Display::fmt(&exp, f)?;
        }

        Ok(())
    }

    fn format_whole(
        &self,
        f: &mut fmt::Formatter<'_>,
        neg: bool,
        mut rem: BigInt,
        div: &BigInt,
        den: &BigInt,
    ) -> fmt::Result {
        if neg {
            f.write_char('-')?;
        }

        fmt::Display::fmt(div, f)?;

        if rem.is_zero() {
            return Ok(());
        }

        if self.spec.limit > 0 {
            f.write_char('.')?;

            for d in emit(&mut rem, den).take(self.spec.limit) {
                fmt::Display::fmt(&d, f)?;
            }
        }

        if !rem.is_zero() && self.spec.cap {
            f.write_char('…')?;
        }

        Ok(())
    }
}

impl fmt::Display for Display<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let neg = self.rational.is_negative();
        let mut rem = self.rational.numer().abs();
        let den = self.rational.denom().abs();

        let div = &rem / &den;
        rem -= &den * &div;

        if digits(div.clone()) >= self.spec.exponent_limit {
            return self.format_big(f, neg, rem, div, &den);
        }

        if !div.is_zero() || rem.is_zero() {
            return self.format_whole(f, neg, rem, &div, &den);
        }

        let mut exp = -1i32;
        let mut init = true;
        let mut dot = true;
        let mut takes_exp = true;
        let mut n = self.spec.limit;

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

                if exp.abs() as usize >= self.spec.exponent_limit {
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

        if !rem.is_zero() && self.spec.cap {
            f.write_char('…')?;
        }

        if exp != 0 {
            write!(f, "e{}", exp)?;
        }

        Ok(())
    }
}

/// Internal helper to keep diving a value and emitting its digits.
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
        debug_assert!(div <= 9);
        Some(div)
    })
}

/// Count the number of digits.
fn digits(mut rem: BigInt) -> usize {
    let mut count = 0;
    rem /= 10u32;

    while !rem.is_zero() {
        rem /= 10u32;
        count += 1;
    }

    count
}
