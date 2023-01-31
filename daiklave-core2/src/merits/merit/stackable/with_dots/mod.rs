use std::ops::Deref;

use crate::{
    merits::merit::{template::MeritTemplateWithDots},
};

mod memo;
pub(crate) use memo::StackableMeritWithDotsMemo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StackableMeritWithDots<'source> {
    Zero(MeritTemplateWithDots<'source>),
    One(MeritTemplateWithDots<'source>),
    Two(MeritTemplateWithDots<'source>),
    Three(MeritTemplateWithDots<'source>),
    Four(MeritTemplateWithDots<'source>),
    Five(MeritTemplateWithDots<'source>),
}

impl<'source> Deref for StackableMeritWithDots<'source> {
    type Target = MeritTemplateWithDots<'source>

    fn deref(&self) -> &Self::Target {
        match self {
            StackableMeritWithDots::Zero(with_dots)
            | StackableMeritWithDots::One(with_dots)
            | StackableMeritWithDots::Two(with_dots)
            | StackableMeritWithDots::Three(with_dots)
            | StackableMeritWithDots::Four(with_dots)
            | StackableMeritWithDots::Five(with_dots) => with_dots,
        }
    }
}

impl<'source> StackableMeritWithDots<'source> {
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
}
