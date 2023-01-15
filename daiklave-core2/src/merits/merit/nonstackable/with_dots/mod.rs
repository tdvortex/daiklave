mod dots;
pub(crate) use dots::{ZeroDotsNonStackableMerit, ZeroDotsNonStackableMeritMemo, OneDotNonStackableMerit, OneDotNonStackableMeritMemo, TwoDotsNonStackableMerit, TwoDotsNonStackableMeritMemo, ThreeDotsNonStackableMerit, ThreeDotsNonStackableMeritMemo, FiveDotsNonStackableMerit, FiveDotsNonStackableMeritMemo, FourDotsNonStackableMerit, FourDotsNonStackableMeritMemo};

mod memo;
pub(crate) use memo::NonStackableMeritWithDotsMemo;

pub(crate) enum NonStackableMeritWithDots<'source> {
    Zero(ZeroDotsNonStackableMerit<'source>),
    One(OneDotNonStackableMerit<'source>),
    Two(TwoDotsNonStackableMerit<'source>),
    Three(ThreeDotsNonStackableMerit<'source>),
    Four(FourDotsNonStackableMerit<'source>),
    Five(FiveDotsNonStackableMerit<'source>),
}

impl<'source> NonStackableMeritWithDots<'source> {
    pub fn template_name(&self) -> &'source str {
        match self {
            NonStackableMeritWithDots::Zero(zero) => zero.template_name(),
            NonStackableMeritWithDots::One(one) => one.template_name(),
            NonStackableMeritWithDots::Two(two) => two.template_name(),
            NonStackableMeritWithDots::Three(three) => three.template_name(),
            NonStackableMeritWithDots::Four(four) => four.template_name(),
            NonStackableMeritWithDots::Five(five) => five.template_name(),
        }
    }
}