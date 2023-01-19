use crate::{
    book_reference::BookReference,
    merits::merit::{template::MeritTemplateId, MeritType, MeritPrerequisite},
};

pub(crate) use self::dots::{
    FiveDotsStackableMerit, FiveDotsStackableMeritMemo, FourDotsStackableMerit,
    FourDotsStackableMeritMemo, OneDotStackableMerit, OneDotStackableMeritMemo,
    ThreeDotsStackableMerit, ThreeDotsStackableMeritMemo, TwoDotsStackableMerit,
    TwoDotsStackableMeritMemo, ZeroDotsStackableMerit, ZeroDotsStackableMeritMemo,
};

mod dots;

mod memo;
pub(crate) use memo::StackableMeritWithDotsMemo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StackableMeritWithDots<'source> {
    Zero(ZeroDotsStackableMerit<'source>),
    One(OneDotStackableMerit<'source>),
    Two(TwoDotsStackableMerit<'source>),
    Three(ThreeDotsStackableMerit<'source>),
    Four(FourDotsStackableMerit<'source>),
    Five(FiveDotsStackableMerit<'source>),
}

impl<'source> StackableMeritWithDots<'source> {
    pub fn as_memo(&self) -> StackableMeritWithDotsMemo {
        match self {
            StackableMeritWithDots::Zero(zero) => StackableMeritWithDotsMemo::Zero(zero.as_memo()),
            StackableMeritWithDots::One(one) => StackableMeritWithDotsMemo::One(one.as_memo()),
            StackableMeritWithDots::Two(two) => StackableMeritWithDotsMemo::Two(two.as_memo()),
            StackableMeritWithDots::Three(three) => {
                StackableMeritWithDotsMemo::Three(three.as_memo())
            }
            StackableMeritWithDots::Four(four) => StackableMeritWithDotsMemo::Four(four.as_memo()),
            StackableMeritWithDots::Five(five) => StackableMeritWithDotsMemo::Five(five.as_memo()),
        }
    }

    pub fn template_id(&self) -> MeritTemplateId {
        match self {
            StackableMeritWithDots::Zero(zero) => zero.template_id(),
            StackableMeritWithDots::One(one) => one.template_id(),
            StackableMeritWithDots::Two(two) => two.template_id(),
            StackableMeritWithDots::Three(three) => three.template_id(),
            StackableMeritWithDots::Four(four) => four.template_id(),
            StackableMeritWithDots::Five(five) => five.template_id(),
        }
    }

    pub fn template_name(&self) -> &'source str {
        match self {
            StackableMeritWithDots::Zero(zero) => zero.name(),
            StackableMeritWithDots::One(one) => one.name(),
            StackableMeritWithDots::Two(two) => two.name(),
            StackableMeritWithDots::Three(three) => three.name(),
            StackableMeritWithDots::Four(four) => four.name(),
            StackableMeritWithDots::Five(five) => five.name(),
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            StackableMeritWithDots::Zero(zero) => zero.book_reference(),
            StackableMeritWithDots::One(one) => one.book_reference(),
            StackableMeritWithDots::Two(two) => two.book_reference(),
            StackableMeritWithDots::Three(three) => three.book_reference(),
            StackableMeritWithDots::Four(four) => four.book_reference(),
            StackableMeritWithDots::Five(five) => five.book_reference(),
        }
    }

    pub fn dots(&self) -> u8 {
        match self {
            StackableMeritWithDots::Zero(_) => 0,
            StackableMeritWithDots::One(_) => 1,
            StackableMeritWithDots::Two(_) => 2,
            StackableMeritWithDots::Three(_) => 3,
            StackableMeritWithDots::Four(_) => 4,
            StackableMeritWithDots::Five(_) => 5,
        }
    }

    pub fn merit_type(&self) -> MeritType {
        match self {
            StackableMeritWithDots::Zero(zero) => zero.merit_type(),
            StackableMeritWithDots::One(one) => one.merit_type(),
            StackableMeritWithDots::Two(two) => two.merit_type(),
            StackableMeritWithDots::Three(three) => three.merit_type(),
            StackableMeritWithDots::Four(four) => four.merit_type(),
            StackableMeritWithDots::Five(five) => five.merit_type(),
        }
    }

    pub fn description(&self) -> (&'source str, Option<&'source str>) {
        match self {
            StackableMeritWithDots::Zero(zero) => zero.description(),
            StackableMeritWithDots::One(one) => one.description(),
            StackableMeritWithDots::Two(two) => two.description(),
            StackableMeritWithDots::Three(three) => three.description(),
            StackableMeritWithDots::Four(four) => four.description(),
            StackableMeritWithDots::Five(five) => five.description(),
        }
    }

    pub fn prerequisites(&self) -> impl ExactSizeIterator<Item = MeritPrerequisite> + '_ {
        match self {
            StackableMeritWithDots::Zero(zero) => zero.prerequisites(),
            StackableMeritWithDots::One(one) => one.prerequisites(),
            StackableMeritWithDots::Two(two) => two.prerequisites(),
            StackableMeritWithDots::Three(three) => three.prerequisites(),
            StackableMeritWithDots::Four(four) => four.prerequisites(),
            StackableMeritWithDots::Five(five) => five.prerequisites(),
        }
    }
}
