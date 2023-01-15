use crate::merits::merit::template::MeritTemplateId;

pub(crate) use self::dots::{
    FiveDotsStackableMerit, FourDotsStackableMerit, OneDotStackableMerit, ThreeDotsStackableMerit,
    TwoDotsStackableMerit, ZeroDotsStackableMerit,
};

mod dots;

mod memo;
pub(crate) use memo::StackableMeritWithDotsMemo;

pub(crate) enum StackableMeritWithDots<'source> {
    Zero(ZeroDotsStackableMerit<'source>),
    One(OneDotStackableMerit<'source>),
    Two(TwoDotsStackableMerit<'source>),
    Three(ThreeDotsStackableMerit<'source>),
    Four(FourDotsStackableMerit<'source>),
    Five(FiveDotsStackableMerit<'source>),
}

impl<'source> StackableMeritWithDots<'source> {
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
            StackableMeritWithDots::Zero(zero) => zero.template_name(),
            StackableMeritWithDots::One(one) => one.template_name(),
            StackableMeritWithDots::Two(two) => two.template_name(),
            StackableMeritWithDots::Three(three) => three.template_name(),
            StackableMeritWithDots::Four(four) => four.template_name(),
            StackableMeritWithDots::Five(five) => five.template_name(),
        }
    }
}