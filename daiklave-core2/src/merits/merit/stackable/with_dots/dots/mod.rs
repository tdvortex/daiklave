use std::ops::Deref;

use crate::merits::merit::template::MeritTemplateWithDots;

mod memo;
pub(crate) use memo::{
    FiveDotsStackableMeritMemo, FourDotsStackableMeritMemo, OneDotStackableMeritMemo,
    ThreeDotsStackableMeritMemo, TwoDotsStackableMeritMemo, ZeroDotsStackableMeritMemo,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ZeroDotsStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> Deref for ZeroDotsStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OneDotStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> Deref for OneDotStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TwoDotsStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> Deref for TwoDotsStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ThreeDotsStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> Deref for ThreeDotsStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FourDotsStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> Deref for FourDotsStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FiveDotsStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> Deref for FiveDotsStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
