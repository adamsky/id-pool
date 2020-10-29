//! Create and recycle integer ids using a ranged pool.
//!
//! This crate is fairly minimalistic and so only deals
//! with single type of numerical ids. These can be either
//! `usize` (default), `u64`, `u32` or `u16`, chosen with
//! the use of appropriate crate feature.
//!
//! The main exported structure [`IdPool`] can be
//! initialized with a custom range and then queried for
//! new ids that are contained within that range. During
//! the course of the program, ids can be returned to the
//! pool to be reused for subsequent id request calls.
//!
//! [`IdPool`]: struct.IdPool.html

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "u16")]
type Num = u16;
#[cfg(feature = "u32")]
type Num = u32;
#[cfg(feature = "u64")]
type Num = u64;
#[cfg(feature = "usize")]
type Num = usize;

/// Custom range struct
#[derive(Copy, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Range {
    start: Num,
    end: Num,
}
impl Range {
    /// Calculates the length of the range.
    pub fn len(&self) -> Num {
        self.end - self.start
    }

    /// Calculates whether a given value is contained
    /// within the range.
    pub fn contains(&self, value: &Num) -> bool {
        // &self.start <= value && value >= &self.end
        value >= &self.start && value <= &self.end
    }
}

/// Keeps track of free ids within a specified range,
/// handles requests and returns of ids based on internal
/// state.
///
/// Internally, a collection of free id ranges is stored.
/// On a request for an id from the pool, the lowest
/// available number will be returned to the caller.
/// Ids can also be returned to the pool to be reused by
/// subsequent id requests.
///
/// # Examples
///
/// ```
/// # use id_pool::IdPool;
/// // initialize a new pool
/// let mut pool = IdPool::new();
/// // request some ids
/// assert_eq!(Some(1), pool.request_id());
/// assert_eq!(Some(2), pool.request_id());
/// assert_eq!(Some(3), pool.request_id());
/// // return the first id
/// assert_eq!(Ok(()), pool.return_id(1));
/// // next request returns recycled first id
/// assert_eq!(Some(1), pool.request_id());
/// // subsequent request returns the next free value
/// assert_eq!(Some(4), pool.request_id());
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IdPool {
    /// List of available id ranges
    free: Vec<Range>,
    /// Number of ids currently in use
    used: usize,
}

impl IdPool {
    /// Creates a new `IdPool` with a default range, which
    /// starts at `1` and ends at `Num::MAX`.
    pub fn new() -> Self {
        Self::new_ranged(1..Num::MAX)
    }

    /// Creates a new `IdPool` with the given range.
    pub fn new_ranged(range: std::ops::Range<Num>) -> Self {
        let vec = vec![Range {
            start: range.start,
            end: range.end,
        }];
        Self { free: vec, used: 0 }
    }

    /// Gets the current count of used ids.
    pub fn get_used(&self) -> usize {
        self.used
    }

    /// Returns a new id or `None` if there are no free ids
    /// in the pool.
    pub fn request_id(&mut self) -> Option<Num> {
        // short-circuit if there are no free ranges
        if self.free.len() == 0 {
            return None;
        }
        // always work on the last range on the list
        let range = self.free.last_mut().unwrap();
        // get the first number from the range
        let id = range.start;
        // increment range starting point
        range.start += 1;
        // if we have just emptied the range then pop it from the list
        if range.len() == 0 {
            self.free.pop();
        }
        self.used += 1;
        Some(id)
    }

    /// Returns an id to the pool or `Err(Num)` if the
    /// id is already in the pool.
    pub fn return_id(&mut self, id: Num) -> Result<(), Num> {
        // search stored ranges for the id in question
        let position = self.free.binary_search_by(|range| {
            // match if the id value is adjacent to the range
            if range.start.checked_sub(1) == Some(id) || range.end == id {
                std::cmp::Ordering::Equal
            }
            // match if the id value is contained within the range
            else if range.contains(&id) {
                std::cmp::Ordering::Equal
            }
            // otherwise indicate the match must be closer to the id value
            else {
                id.cmp(&range.start)
            }
        });
        match position {
            // range containing id in question was not found,
            // insert a new range that includes the returned id
            // at a point in the list that is closest to the id value
            Err(i) => self.free.insert(
                i,
                Range {
                    start: id,
                    end: id + 1,
                },
            ),
            // found range adjacent to or containing the id in question
            Ok(i) => {
                let range = &mut self.free[i];
                // id value adjacent to range start point
                if range.start.checked_sub(1) == Some(id) {
                    range.start = id;
                }
                // id value adjacent to range end point
                else if range.end == id {
                    range.end = range.end + 1;
                }
                // id value contained within one of the ranges,
                // can't return id to the pool
                else {
                    return Err(id);
                }
                // check if there exists a range before the current one
                let before_range_idx = match i.checked_sub(1) {
                    Some(idx) => idx,
                    None => return Err(id),
                };
                // if the current range's end point is the previous range's
                // start point, then merge the ranges into one
                if self.free[before_range_idx].start == self.free[i].end {
                    self.free[before_range_idx].start = self.free[i].start;
                    self.free.remove(i);
                }
            }
        }
        self.used -= 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request() {
        let mut pool = IdPool::new();
        assert_eq!(Some(1), pool.request_id());
        assert_eq!(Some(2), pool.request_id());
        assert_eq!(Some(3), pool.request_id());
    }

    #[test]
    fn request_return() {
        let mut pool = IdPool::new_ranged(1..Num::MAX);
        assert_eq!(Some(1), pool.request_id());
        assert_eq!(Some(2), pool.request_id());
        assert_eq!(Some(3), pool.request_id());
        assert_eq!(Ok(()), pool.return_id(1));
        assert_eq!(Some(1), pool.request_id());
        assert_eq!(Ok(()), pool.return_id(2));
        assert_eq!(Some(2), pool.request_id());
        assert_eq!(Some(4), pool.request_id());
        assert_eq!(Ok(()), pool.return_id(3));
        assert_eq!(Ok(()), pool.return_id(4));
        assert_eq!(Err(5), pool.return_id(5));
    }

    #[test]
    fn used_count() {
        let mut pool = IdPool::new_ranged(1..10);
        assert_eq!(Some(1), pool.request_id());
        assert_eq!(Some(2), pool.request_id());
        assert_eq!(Some(3), pool.request_id());
        assert_eq!(Ok(()), pool.return_id(1));
        assert_eq!(2, pool.get_used());
    }
}
