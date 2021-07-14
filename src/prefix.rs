use std::fmt;

const PREFIXES: [(i32, Prefix); 19] = [
    (Prefix::YOCTO, Prefix::Yocto),
    (Prefix::ZEPTO, Prefix::Zepto),
    (Prefix::ATTO, Prefix::Atto),
    (Prefix::FEMTO, Prefix::Femto),
    (Prefix::PICO, Prefix::Pico),
    (Prefix::NANO, Prefix::Nano),
    (Prefix::MICRO, Prefix::Micro),
    (Prefix::MILLI, Prefix::Milli),
    (Prefix::CENTI, Prefix::Centi),
    (Prefix::DECI, Prefix::Deci),
    (Prefix::NONE, Prefix::None),
    (Prefix::KILO, Prefix::Kilo),
    (Prefix::MEGA, Prefix::Mega),
    (Prefix::GIGA, Prefix::Giga),
    (Prefix::TERA, Prefix::Tera),
    (Prefix::PETA, Prefix::Peta),
    (Prefix::EXA, Prefix::Exa),
    (Prefix::ZETTA, Prefix::Zetta),
    (Prefix::YOTTA, Prefix::Yotta),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Prefix {
    Yotta,
    Zetta,
    Exa,
    Peta,
    Tera,
    Giga,
    Mega,
    Kilo,
    /// Empty prefix.
    None,
    Deci,
    Centi,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
    Atto,
    Zepto,
    Yocto,
}

impl Prefix {
    pub const YOTTA: i32 = 24;
    pub const ZETTA: i32 = 21;
    pub const EXA: i32 = 18;
    pub const PETA: i32 = 15;
    pub const TERA: i32 = 12;
    pub const GIGA: i32 = 9;
    pub const MEGA: i32 = 6;
    pub const KILO: i32 = 3;
    pub const NONE: i32 = 0;
    pub const DECI: i32 = -1;
    pub const CENTI: i32 = -2;
    pub const MILLI: i32 = -3;
    pub const MICRO: i32 = -6;
    pub const NANO: i32 = -9;
    pub const PICO: i32 = -12;
    pub const FEMTO: i32 = -15;
    pub const ATTO: i32 = -18;
    pub const ZEPTO: i32 = -21;
    pub const YOCTO: i32 = -24;

    /// Find the prefix matching the given power and return any extra that comes
    /// along.
    pub fn find(pow: i32) -> (Self, i32) {
        let (p, prefix) = match PREFIXES.binary_search_by(|e| e.0.cmp(&pow)) {
            Ok(n) => PREFIXES[n],
            Err(n) => PREFIXES[n.saturating_sub(1)],
        };

        (prefix, p - pow)
    }
}

impl Default for Prefix {
    fn default() -> Self {
        Prefix::None
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prefix::Yotta => 'Y'.fmt(f),
            Prefix::Zetta => 'Z'.fmt(f),
            Prefix::Exa => 'E'.fmt(f),
            Prefix::Peta => 'P'.fmt(f),
            Prefix::Tera => 'T'.fmt(f),
            Prefix::Giga => 'G'.fmt(f),
            Prefix::Mega => 'M'.fmt(f),
            Prefix::Kilo => 'k'.fmt(f),
            Prefix::None => Ok(()),
            Prefix::Deci => 'd'.fmt(f),
            Prefix::Centi => 'c'.fmt(f),
            Prefix::Milli => 'm'.fmt(f),
            Prefix::Micro => 'μ'.fmt(f),
            Prefix::Nano => 'n'.fmt(f),
            Prefix::Pico => 'p'.fmt(f),
            Prefix::Femto => 'f'.fmt(f),
            Prefix::Atto => 'a'.fmt(f),
            Prefix::Zepto => 'z'.fmt(f),
            Prefix::Yocto => 'y'.fmt(f),
        }
    }
}
