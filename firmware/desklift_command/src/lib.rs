#![cfg_attr(not(test), no_std)]

use core::fmt;


#[derive(PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Direction::Up => "up",
            Direction::Down => "down",
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Command(i8);

impl Command {
    pub fn new(byte: i8) -> Self {
        Command(byte)
    }

    pub fn from_u8(byte: u8) -> Self {
        Command(byte as i8)
    }

    pub fn get_ms(&self) -> u16 {
        if self.0 == -128 {
            // Absolute value of -128 doesn't fit into an i8
            1280
        } else {
            (self.0.abs() as u16) * 10
        }
    }

    pub fn get_direction(&self) -> Direction {
        if self.0 < 0 {
            Direction::Down
        } else {
            Direction::Up
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}ms", &self.get_direction(), self.get_ms())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        assert_eq!(Command::from_u8(0), Command::new(0));
        assert_eq!(Command::from_u8(10), Command::new(10));
        assert_eq!(Command::from_u8(128), Command::new(-128));
        assert_eq!(Command::from_u8(129), Command::new(-127));
        assert_eq!(Command::from_u8(255), Command::new(-1));
    }

    #[test]
    fn test_direction() {
        for i in 0..127 {
            assert_eq!(Command::new(i).get_direction(), Direction::Up);
        }
        for i in -128..-1 {
            assert_eq!(Command::new(i).get_direction(), Direction::Down);
        }
    }

    #[test]
    fn test_duration() {
        for i in 0..127 {
            assert_eq!(Command::new(i).get_ms(), (i as u16) * 10);
        }
        for i in 0..128 {
            assert_eq!(Command::new(-i as i8).get_ms(), (i as u16) * 10);
        }
    }
}
