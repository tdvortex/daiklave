use serde::{Deserialize, Serialize};

use crate::merits::merit::MeritPrerequisite;

use super::{
    dots::{
        FiveDotsStackableMeritMemo, FourDotsStackableMeritMemo, OneDotStackableMeritMemo,
        ThreeDotsStackableMeritMemo, TwoDotsStackableMeritMemo, ZeroDotsStackableMeritMemo,
    },
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
    pub fn prerequisites(&self) -> impl ExactSizeIterator<Item = MeritPrerequisite> + '_ {
        match self {
            StackableMeritWithDotsMemo::Zero(zero) => zero.0.prerequisites.iter().copied(),
            StackableMeritWithDotsMemo::One(one) => one.0.prerequisites.iter().copied(),
            StackableMeritWithDotsMemo::Two(two) => two.0.prerequisites.iter().copied(),
            StackableMeritWithDotsMemo::Three(three) => three.0.prerequisites.iter().copied(),
            StackableMeritWithDotsMemo::Four(four) => four.0.prerequisites.iter().copied(),
            StackableMeritWithDotsMemo::Five(five) => five.0.prerequisites.iter().copied(),
        }
    }
}
