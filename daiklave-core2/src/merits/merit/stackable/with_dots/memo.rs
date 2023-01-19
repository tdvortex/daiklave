use serde::{Deserialize, Serialize};

use crate::merits::merit::MeritPrerequisite;

use super::{
    dots::{
        FiveDotsStackableMeritMemo, FourDotsStackableMeritMemo, OneDotStackableMeritMemo,
        ThreeDotsStackableMeritMemo, TwoDotsStackableMeritMemo, ZeroDotsStackableMeritMemo,
    },
    StackableMeritWithDots,
};

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

    pub fn prerequisites(&self) -> impl ExactSizeIterator<Item = MeritPrerequisite> + '_ {
        match self {
            StackableMeritWithDotsMemo::Zero(zero) => zero.1.prerequisites.iter().copied(),
            StackableMeritWithDotsMemo::One(one) => one.1.prerequisites.iter().copied(),
            StackableMeritWithDotsMemo::Two(two) => two.1.prerequisites.iter().copied(),
            StackableMeritWithDotsMemo::Three(three) => three.1.prerequisites.iter().copied(),
            StackableMeritWithDotsMemo::Four(four) => four.1.prerequisites.iter().copied(),
            StackableMeritWithDotsMemo::Five(five) => five.1.prerequisites.iter().copied(),
        }
    }
}
