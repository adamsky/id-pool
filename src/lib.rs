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

#[cfg(feature = "u16")]
type Num = u16;
#[cfg(feature = "u32")]
type Num = u32;
#[cfg(feature = "u64")]
type Num = u64;
#[cfg(feature = "usize")]
type Num = usize;

/// Encapsulate `Range` type over `Num`.
type Range = std::ops::Range<Num>;

/// Keeps track of free ids within a specified range,
/// handles requests and returns of ids based on internal
/// state.
///
/// Internally, a collection of free id ranges is stored.
/// On a request for an id from the pool the lowest
/// available number will be returned to the caller.
/// Ids can also be returned to the pool to be reused by
/// subsequent id requests.
///
/// # Examples
///
/// ```
/// # use id_pool::IdPool;
/// // initialize a new pool ranged 1 to 10
/// let mut pool = IdPool::new_ranged(1..10);
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
pub struct IdPool {
    /// List of available id ranges
    free: Vec<Range>,
}

impl IdPool {
    /// Creates a new `IdPool` with a default range, which
    /// starts at `1` and ends at `Num::MAX`.
    pub fn new() -> Self {
        Self {
            free: vec![1..Num::MAX],
        }
    }

    /// Creates a new `IdPool` with the given range.
    pub fn new_ranged(range: Range) -> Self {
        Self { free: vec![range] }
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
        range.start = range.start + 1;
        // if we have just emptied the range then pop it from the list
        if range.len() == 0 {
            self.free.pop();
        }
        Some(id)
    }

    /// Returns an id to the pool or `Err(Num)` if the
    /// id is already in the pool.
    pub fn return_id(&mut self, id: Num) -> Result<(), Num> {
        // search stored ranges for the id in question
        let position = self.free.binary_search_by(|range| {
            // match if the id value is contained within the range
            if range.contains(&id) {
                std::cmp::Ordering::Equal
            }
            // also match if the id value is adjacent to the range
            else if range.start.checked_sub(1) == Some(id) || range.end == id {
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
}
