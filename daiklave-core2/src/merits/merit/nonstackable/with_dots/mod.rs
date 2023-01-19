mod dots;
pub(crate) use dots::{
    FiveDotsNonStackableMerit, FiveDotsNonStackableMeritMemo, FourDotsNonStackableMerit,
    FourDotsNonStackableMeritMemo, OneDotNonStackableMerit, OneDotNonStackableMeritMemo,
    ThreeDotsNonStackableMerit, ThreeDotsNonStackableMeritMemo, TwoDotsNonStackableMerit,
    TwoDotsNonStackableMeritMemo, ZeroDotsNonStackableMerit, ZeroDotsNonStackableMeritMemo,
};

mod memo;
pub(crate) use memo::NonStackableMeritWithDotsMemo;

use crate::{book_reference::BookReference, merits::merit::{MeritType, MeritPrerequisite}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NonStackableMeritWithDots<'source> {
    Zero(ZeroDotsNonStackableMerit<'source>),
    One(OneDotNonStackableMerit<'source>),
    Two(TwoDotsNonStackableMerit<'source>),
    Three(ThreeDotsNonStackableMerit<'source>),
    Four(FourDotsNonStackableMerit<'source>),
    Five(FiveDotsNonStackableMerit<'source>),
}

impl<'source> NonStackableMeritWithDots<'source> {
    pub fn as_memo(&self) -> NonStackableMeritWithDotsMemo {
        match self {
            NonStackableMeritWithDots::Zero(zero) => {
                NonStackableMeritWithDotsMemo::Zero(zero.as_memo())
            }
            NonStackableMeritWithDots::One(one) => {
                NonStackableMeritWithDotsMemo::One(one.as_memo())
            }
            NonStackableMeritWithDots::Two(two) => {
                NonStackableMeritWithDotsMemo::Two(two.as_memo())
            }
            NonStackableMeritWithDots::Three(three) => {
                NonStackableMeritWithDotsMemo::Three(three.as_memo())
            }
            NonStackableMeritWithDots::Four(four) => {
                NonStackableMeritWithDotsMemo::Four(four.as_memo())
            }
            NonStackableMeritWithDots::Five(five) => {
                NonStackableMeritWithDotsMemo::Five(five.as_memo())
            }
        }
    }

    pub fn template_name(&self) -> &'source str {
        match self {
            NonStackableMeritWithDots::Zero(zero) => zero.name(),
            NonStackableMeritWithDots::One(one) => one.name(),
            NonStackableMeritWithDots::Two(two) => two.name(),
            NonStackableMeritWithDots::Three(three) => three.name(),
            NonStackableMeritWithDots::Four(four) => four.name(),
            NonStackableMeritWithDots::Five(five) => five.name(),
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            NonStackableMeritWithDots::Zero(zero) => zero.book_reference(),
            NonStackableMeritWithDots::One(one) => one.book_reference(),
            NonStackableMeritWithDots::Two(two) => two.book_reference(),
            NonStackableMeritWithDots::Three(three) => three.book_reference(),
            NonStackableMeritWithDots::Four(four) => four.book_reference(),
            NonStackableMeritWithDots::Five(five) => five.book_reference(),
        }
    }

    pub fn dots(&self) -> u8 {
        match self {
            NonStackableMeritWithDots::Zero(_) => 0,
            NonStackableMeritWithDots::One(_) => 1,
            NonStackableMeritWithDots::Two(_) => 2,
            NonStackableMeritWithDots::Three(_) => 3,
            NonStackableMeritWithDots::Four(_) => 4,
            NonStackableMeritWithDots::Five(_) => 5,
        }
    }

    pub fn merit_type(&self) -> MeritType {
        match self {
            NonStackableMeritWithDots::Zero(zero) => zero.merit_type(),
            NonStackableMeritWithDots::One(one) => one.merit_type(),
            NonStackableMeritWithDots::Two(two) => two.merit_type(),
            NonStackableMeritWithDots::Three(three) => three.merit_type(),
            NonStackableMeritWithDots::Four(four) => four.merit_type(),
            NonStackableMeritWithDots::Five(five) => five.merit_type(),
        }
    }

    pub fn description(&self) -> (&'source str, Option<&'source str>) {
        match self {
            NonStackableMeritWithDots::Zero(zero) => zero.description(),
            NonStackableMeritWithDots::One(one) => one.description(),
            NonStackableMeritWithDots::Two(two) => two.description(),
            NonStackableMeritWithDots::Three(three) => three.description(),
            NonStackableMeritWithDots::Four(four) => four.description(),
            NonStackableMeritWithDots::Five(five) => five.description(),
        }
    }

    pub fn prerequisites(&self) -> impl ExactSizeIterator<Item = MeritPrerequisite> + '_ {
        match self {
            NonStackableMeritWithDots::Zero(zero) => zero.prerequisites(),
            NonStackableMeritWithDots::One(one) => one.prerequisites(),
            NonStackableMeritWithDots::Two(two) => two.prerequisites(),
            NonStackableMeritWithDots::Three(three) => three.prerequisites(),
            NonStackableMeritWithDots::Four(four) => four.prerequisites(),
            NonStackableMeritWithDots::Five(five) => five.prerequisites(),
        }
    }
}
