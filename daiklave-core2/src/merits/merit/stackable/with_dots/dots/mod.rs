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

impl<'source> ZeroDotsStackableMerit<'source> {
    pub fn as_memo(&self) -> ZeroDotsStackableMeritMemo {
        ZeroDotsStackableMeritMemo(self.0, self.1.as_memo())
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

impl<'source> OneDotStackableMerit<'source> {
    pub fn as_memo(&self) -> OneDotStackableMeritMemo {
        OneDotStackableMeritMemo(self.0, self.1.as_memo())
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

impl<'source> TwoDotsStackableMerit<'source> {
    pub fn as_memo(&self) -> TwoDotsStackableMeritMemo {
        TwoDotsStackableMeritMemo(self.0, self.1.as_memo())
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

impl<'source> ThreeDotsStackableMerit<'source> {
    pub fn as_memo(&self) -> ThreeDotsStackableMeritMemo {
        ThreeDotsStackableMeritMemo(self.0, self.1.as_memo())
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

impl<'source> FourDotsStackableMerit<'source> {
    pub fn as_memo(&self) -> FourDotsStackableMeritMemo {
        FourDotsStackableMeritMemo(self.0, self.1.as_memo())
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

impl<'source> FiveDotsStackableMerit<'source> {
    pub fn as_memo(&self) -> FiveDotsStackableMeritMemo {
        FiveDotsStackableMeritMemo(self.0, self.1.as_memo())
    }
}
