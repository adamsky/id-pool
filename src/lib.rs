//! Create and recycle integer ids using a ranged pool.

type Range = std::ops::Range<usize>;

/// Handles requests and returns of ids based on internal
/// state.
#[derive(Debug, Clone)]
pub struct IdPool {
    /// List of available id ranges
    free: Vec<Range>,
}

impl IdPool {
    /// Creates a new `IdPool` with a default range.
    pub fn new() -> Self {
        Self {
            free: vec![1..usize::MAX],
        }
    }

    /// Creates a new `IdPool` with the given range.
    pub fn new_ranged(range: Range) -> Self {
        Self { free: vec![range] }
    }

    /// Returns a new id.
    pub fn request_id(&mut self) -> Option<usize> {
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
        Some(id)
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
}