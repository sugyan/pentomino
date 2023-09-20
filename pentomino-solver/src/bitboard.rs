use derive_more::{BitAnd, BitAndAssign, BitOr, BitOrAssign, From, Into};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    // From/Into, and bit manipulations by derive_more
    From,
    Into,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn is_empty(&self) -> bool {
        self.0 == 0
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
