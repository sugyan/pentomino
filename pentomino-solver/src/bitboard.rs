use std::ops::{BitAnd, BitOr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Bitboard> for u64 {
    fn from(b: Bitboard) -> Self {
        b.0
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty() {
        assert!(Bitboard::from(0).is_empty());
        assert!(!Bitboard::from(1).is_empty());
    }

    #[test]
    fn from() {
        assert_eq!(Bitboard::from(0), Bitboard(0));
        assert_eq!(Bitboard::from(1), Bitboard(1));
    }

    #[test]
    fn into() {
        assert_eq!(u64::from(Bitboard(0)), 0);
        assert_eq!(u64::from(Bitboard(1)), 1);
    }

    #[test]
    fn bitand() {
        assert_eq!(Bitboard(0) & Bitboard(0), Bitboard(0));
        assert_eq!(Bitboard(0) & Bitboard(1), Bitboard(0));
        assert_eq!(Bitboard(1) & Bitboard(0), Bitboard(0));
        assert_eq!(Bitboard(1) & Bitboard(1), Bitboard(1));
    }

    #[test]
    fn bitor() {
        assert_eq!(Bitboard(0) | Bitboard(0), Bitboard(0));
        assert_eq!(Bitboard(0) | Bitboard(1), Bitboard(1));
        assert_eq!(Bitboard(1) | Bitboard(0), Bitboard(1));
        assert_eq!(Bitboard(1) | Bitboard(1), Bitboard(1));
    }
}
