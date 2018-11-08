use crate::index::Index;


const SIZE: usize = 64 + 1;


/// A lock-free ring buffer / circular buffer.
///
/// To differentiate between a full and an empty buffer, only SIZE-1 elements
/// can be stored in the buffer.
pub struct RingBuf {
    /// The byte buffer.
    buf: [u8; SIZE],

    /// The read index.
    read_index: Index,

    /// The write index.
    write_index: Index,
}

#[derive(Debug, PartialEq)]
pub enum RingBufError {
    Full
}

impl RingBuf {
    pub fn new() -> Self {
        RingBuf {
            buf: [0; SIZE],
            read_index: Index::new(SIZE),
            write_index: Index::new(SIZE),
        }
    }

    /// Add a byte to the buffer.
    pub fn push(&mut self, byte: u8) -> Result<(), RingBufError> {
        // Ensure the buffer is not already full
        if self.full() {
            return Err(RingBufError::Full);
        }

        // Store the byte
        self.buf[self.write_index.current()] = byte;

        // Increment the write pointer
        self.write_index.increment();

        Ok(())
    }

    /// Read a byte from the buffer.
    pub fn pop(&mut self) -> Option<u8> {
        if self.empty() {
            None
        } else {
            let val = self.buf[self.read_index.current()];
            self.read_index.increment();
            Some(val)
        }
    }

    /// The buffer is full if the write index is 1 position behind the read index.
    pub fn full(&self) -> bool {
        self.write_index.next() == self.read_index.current()
    }

    /// The buffer is empty if the read index and the write index are equal.
    pub fn empty(&self) -> bool {
        self.read_index.current() == self.write_index.current()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut buf = RingBuf::new();
        assert!(buf.empty());
        buf.push(1).unwrap();
        assert!(!buf.empty());
        buf.pop();
        assert!(buf.empty());
    }

    #[test]
    fn full() {
        let mut buf = RingBuf::new();
        assert!(!buf.full());

        // Add 63 entries
        for i in 1..=63 {
            buf.push(i).unwrap();
        }
        assert!(!buf.full());

        // Add the 64th entry
        buf.push(64).unwrap();
        assert!(buf.full());
    }

    #[test]
    fn pop() {
        let mut buf = RingBuf::new();

        // Initially empty
        assert_eq!(buf.pop(), None);

        // Insert two values
        buf.push(1).unwrap();
        buf.push(2).unwrap();

        // Read first value
        assert_eq!(buf.pop(), Some(1));

        // Add some more
        buf.push(3).unwrap();

        // Read more values
        assert_eq!(buf.pop(), Some(2));
        assert_eq!(buf.pop(), Some(3));
        assert_eq!(buf.pop(), None);
    }

    #[test]
    fn write() {
        let mut buf = RingBuf::new();

        // Add 64 entries
        for i in 1..=64 {
            assert_eq!(buf.push(i), Ok(()));
        }

        // Adding the 65th entry will fail
        assert_eq!(buf.push(65), Err(RingBufError::Full));

        // But when reading 1 item, another slot will be freed
        assert_eq!(buf.pop(), Some(1));
        assert_eq!(buf.push(65), Ok(()));
        assert_eq!(buf.push(66), Err(RingBufError::Full));
    }
}
