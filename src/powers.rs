use crate::unit::Unit;
use std::collections::{btree_map, BTreeMap};

/// Helpers struct to build bases.
///
/// ```
/// use facts::{Unit, Powers};
///
/// let mut powers = Powers::default();
/// powers.insert(Unit::Meter, 2);
/// powers.insert(Unit::Second, 1);
/// powers.insert(Unit::Second, -2);
///
/// assert_eq!(powers.get(Unit::Meter), Some(2));
/// assert_eq!(powers.get(Unit::Second), Some(-1));
/// ```
#[derive(Default)]
pub struct Powers {
    powers: BTreeMap<Unit, i32>,
}

impl Powers {
    /// Test if powers is empty.
    ///
    /// ```
    /// use facts::Powers;
    ///
    /// let powers = Powers::default();
    /// assert!(powers.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.powers.is_empty()
    }

    /// Get the number of units in this collection.
    ///
    /// ```
    /// use facts::{Unit, Powers};
    ///
    /// let mut powers = Powers::default();
    /// assert_eq!(powers.len(), 0);
    ///
    /// powers.insert(Unit::Meter, 2);
    /// powers.insert(Unit::Second, 1);
    /// powers.insert(Unit::Second, -2);
    ///
    /// assert_eq!(powers.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.powers.len()
    }

    /// Clear the current collection of powers.
    ///
    /// ```
    /// use facts::{Unit, Powers};
    ///
    /// let mut powers = Powers::default();
    /// powers.insert(Unit::Meter, 2);
    /// powers.insert(Unit::Second, 1);
    /// powers.insert(Unit::Second, -2);
    ///
    /// assert_eq!(powers.get(Unit::Meter), Some(2));
    /// assert_eq!(powers.get(Unit::Second), Some(-1));
    ///
    /// assert_eq!(powers.len(), 2);
    /// powers.clear();
    /// assert_eq!(powers.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.powers.clear();
    }

    /// Get the power associated with the unit.
    ///
    /// ```
    /// use facts::{Unit, Powers};
    ///
    /// let mut powers = Powers::default();
    /// powers.insert(Unit::Meter, 2);
    /// powers.insert(Unit::Second, 1);
    /// powers.insert(Unit::Second, -2);
    ///
    /// assert_eq!(powers.get(Unit::Meter), Some(2));
    /// assert_eq!(powers.get(Unit::Second), Some(-1));
    /// ```
    pub fn get(&self, unit: Unit) -> Option<i32> {
        self.powers.get(&unit).copied()
    }

    /// Insert the given unit to the collection of powers.
    ///
    /// This will accumulate the specified unit onto the existing powers.
    ///
    /// ```
    /// use facts::{Unit, Powers};
    ///
    /// let mut powers = Powers::default();
    /// powers.insert(Unit::Meter, 2);
    /// powers.insert(Unit::Second, 1);
    /// powers.insert(Unit::Second, -2);
    ///
    /// assert_eq!(powers.get(Unit::Meter), Some(2));
    /// assert_eq!(powers.get(Unit::Second), Some(-1));
    /// ```
    pub fn insert(&mut self, unit: Unit, power: i32) {
        match self.powers.entry(unit) {
            btree_map::Entry::Vacant(e) => {
                e.insert(power);
            }
            btree_map::Entry::Occupied(mut e) => {
                *e.get_mut() += power;
            }
        }
    }

    /// Iterate over all populated powers.
    ///
    /// ```
    /// use facts::{Unit, Powers};
    ///
    /// let mut powers = Powers::default();
    /// powers.insert(Unit::Meter, 2);
    /// powers.insert(Unit::Second, 1);
    /// powers.insert(Unit::Second, -5);
    ///
    /// assert_eq! {
    ///     powers.iter().collect::<Vec<_>>(),
    ///     &[(Unit::Meter, 2), (Unit::Second, -4)],
    /// };
    /// ```
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            iter: self.powers.iter(),
        }
    }
}

impl<'a> IntoIterator for &'a Powers {
    type Item = (Unit, i32);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for Powers {
    type Item = (Unit, i32);
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.powers.into_iter(),
        }
    }
}

/// The iterator over the existing powers.
///
/// See [Powers::iter].
pub struct Iter<'a> {
    iter: btree_map::Iter<'a, Unit, i32>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (Unit, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let (unit, power) = self.iter.next()?;
        Some((*unit, *power))
    }
}

/// The iterator over the existing powers.
///
/// See [Powers::into_iter].
pub struct IntoIter {
    iter: btree_map::IntoIter<Unit, i32>,
}

impl<'a> Iterator for IntoIter {
    type Item = (Unit, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
