/// A wrapping integer. It can be incremented, but is reset to 0 before
/// reaching the limit.
pub struct Index {
    limit: usize,
    val: usize,
}

impl Index {
    /// Create a new `Index` instance. The value will wrap at the limit
    /// (meaning that it will always be smaller than the limit).
    pub const fn new(limit: usize) -> Self {
        Index {
            limit,
            val: 0,
        }
    }

    /// Increment the index. This will wrap back to 0 if the limit is reached.
    /// The limit is exclusive, the value range is from `0` to `limit-1`.
    pub fn increment(&mut self) {
        self.val = self.next();
    }

    /// Return the current index value.
    pub fn current(&self) -> usize {
        self.val
    }

    /// Return the next number without modifying the internal value.
    /// The limit is exclusive, the value range is from `0` to `limit-1`.
    pub fn next(&self) -> usize {
        if self.val + 1 >= self.limit {
            0
        } else {
            self.val + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Index;

    #[test]
    fn index_wrapping() {
        let mut index3 = Index::new(3);
        let mut index4 = Index::new(4);
        assert_eq!(index3.current(), 0);
        assert_eq!(index4.current(), 0);

        index3.increment(); index4.increment();
        index3.increment(); index4.increment();
        assert_eq!(index3.current(), 2);
        assert_eq!(index4.current(), 2);

        index3.increment(); index4.increment();
        assert_eq!(index3.current(), 0);
        assert_eq!(index4.current(), 3);

        index3.increment(); index4.increment();
        assert_eq!(index3.current(), 1);
        assert_eq!(index4.current(), 0);
    }

    #[test]
    fn next() {
        let mut index3 = Index::new(3);

        // next() should return the next value...
        assert_eq!(index3.current(), 0);
        assert_eq!(index3.next(), 1);

        // ...without modifying the current value.
        assert_eq!(index3.current(), 0);

        // Wrapping should be taken into account.
        index3.increment(); // 1
        index3.increment(); // 2
        assert_eq!(index3.current(), 2);
        assert_eq!(index3.next(), 0);
    }
}
