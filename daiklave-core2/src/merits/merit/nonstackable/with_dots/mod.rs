use std::ops::Deref;

mod memo;
pub(crate) use memo::NonStackableMeritWithDotsMemo;

use crate::{
    merits::merit::{template::MeritTemplateWithDots},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NonStackableMeritWithDots<'source> {
    Zero(MeritTemplateWithDots<'source>),
    One(MeritTemplateWithDots<'source>),
    Two(MeritTemplateWithDots<'source>),
    Three(MeritTemplateWithDots<'source>),
    Four(MeritTemplateWithDots<'source>),
    Five(MeritTemplateWithDots<'source>),
}

impl<'source> Deref for NonStackableMeritWithDots<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        match self {
            NonStackableMeritWithDots::Zero(with_dots)
            | NonStackableMeritWithDots::One(with_dots)
            | NonStackableMeritWithDots::Two(with_dots)
            | NonStackableMeritWithDots::Three(with_dots)
            | NonStackableMeritWithDots::Four(with_dots)
            | NonStackableMeritWithDots::Five(with_dots) => with_dots,
        }
    }
}



impl<'source> NonStackableMeritWithDots<'source> {
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
}
