use serde::{Serialize, Deserialize};

use super::{ZeroDotsNonStackableMeritMemo, OneDotNonStackableMeritMemo, TwoDotsNonStackableMeritMemo, ThreeDotsNonStackableMeritMemo, FourDotsNonStackableMeritMemo, FiveDotsNonStackableMeritMemo, NonStackableMeritWithDots};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum NonStackableMeritWithDotsMemo {
    Zero(ZeroDotsNonStackableMeritMemo),
    One(OneDotNonStackableMeritMemo),
    Two(TwoDotsNonStackableMeritMemo),
    Three(ThreeDotsNonStackableMeritMemo),
    Four(FourDotsNonStackableMeritMemo),
    Five(FiveDotsNonStackableMeritMemo),
}

impl<'source> NonStackableMeritWithDotsMemo {
    pub fn as_ref(&'source self) -> NonStackableMeritWithDots<'source> {
        match self {
            NonStackableMeritWithDotsMemo::Zero(zero) => NonStackableMeritWithDots::Zero(zero.as_ref()),
            NonStackableMeritWithDotsMemo::One(one) => NonStackableMeritWithDots::One(one.as_ref()),
            NonStackableMeritWithDotsMemo::Two(two) => NonStackableMeritWithDots::Two(two.as_ref()),
            NonStackableMeritWithDotsMemo::Three(three) => NonStackableMeritWithDots::Three(three.as_ref()),
            NonStackableMeritWithDotsMemo::Four(four) => NonStackableMeritWithDots::Four(four.as_ref()),
            NonStackableMeritWithDotsMemo::Five(five) => NonStackableMeritWithDots::Five(five.as_ref()),
        }
    }
}