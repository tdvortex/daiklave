use serde::{Serialize, Deserialize};

use super::{dots::{ZeroDotsStackableMeritMemo, OneDotStackableMeritMemo, TwoDotsStackableMeritMemo, ThreeDotsStackableMeritMemo, FourDotsStackableMeritMemo, FiveDotsStackableMeritMemo}, StackableMeritWithDots};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum StackableMeritWithDotsMemo {
    Zero(ZeroDotsStackableMeritMemo),
    One(OneDotStackableMeritMemo),
    Two(TwoDotsStackableMeritMemo),
    Three(ThreeDotsStackableMeritMemo),
    Four(FourDotsStackableMeritMemo),
    Five(FiveDotsStackableMeritMemo),
}

impl<'source> StackableMeritWithDotsMemo {
    pub fn as_ref(&'source self) -> StackableMeritWithDots<'source> {
        match self {
            StackableMeritWithDotsMemo::Zero(memo) => StackableMeritWithDots::Zero(memo.as_ref()),
            StackableMeritWithDotsMemo::One(memo) => StackableMeritWithDots::One(memo.as_ref()),
            StackableMeritWithDotsMemo::Two(memo) => StackableMeritWithDots::Two(memo.as_ref()),
            StackableMeritWithDotsMemo::Three(memo) => StackableMeritWithDots::Three(memo.as_ref()),
            StackableMeritWithDotsMemo::Four(memo) => StackableMeritWithDots::Four(memo.as_ref()),
            StackableMeritWithDotsMemo::Five(memo) => StackableMeritWithDots::Five(memo.as_ref()),
        }
    }
}