#[derive(Debug, Clone)]
pub struct IdPool {
    free: Vec<std::ops::Range<usize>>,
}

impl IdPool {
    pub fn new() -> Self {
        Self {
            free: vec![1..usize::MAX],
        }
    }

    pub fn request_id(&mut self) -> Option<usize> {
        let range = self.free.last_mut().unwrap();
        let id = range.start;
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