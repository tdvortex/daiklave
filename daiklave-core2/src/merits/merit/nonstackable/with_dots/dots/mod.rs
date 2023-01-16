use std::ops::Deref;

use crate::merits::merit::template::MeritTemplateWithDots;

mod memo;
pub(crate) use memo::{
    FiveDotsNonStackableMeritMemo, FourDotsNonStackableMeritMemo, OneDotNonStackableMeritMemo,
    ThreeDotsNonStackableMeritMemo, TwoDotsNonStackableMeritMemo, ZeroDotsNonStackableMeritMemo,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ZeroDotsNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> ZeroDotsNonStackableMerit<'source> {
    pub fn as_memo(&self) -> ZeroDotsNonStackableMeritMemo {
        ZeroDotsNonStackableMeritMemo(self.0.as_memo())
    }
}

impl<'source> Deref for ZeroDotsNonStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OneDotNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> OneDotNonStackableMerit<'source> {
    pub fn as_memo(&self) -> OneDotNonStackableMeritMemo {
        OneDotNonStackableMeritMemo(self.0.as_memo())
    }
}

impl<'source> Deref for OneDotNonStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TwoDotsNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> TwoDotsNonStackableMerit<'source> {
    pub fn as_memo(&self) -> TwoDotsNonStackableMeritMemo {
        TwoDotsNonStackableMeritMemo(self.0.as_memo())
    }
}

impl<'source> Deref for TwoDotsNonStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ThreeDotsNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> ThreeDotsNonStackableMerit<'source> {
    pub fn as_memo(&self) -> ThreeDotsNonStackableMeritMemo {
        ThreeDotsNonStackableMeritMemo(self.0.as_memo())
    }
}

impl<'source> Deref for ThreeDotsNonStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FourDotsNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> FourDotsNonStackableMerit<'source> {
    pub fn as_memo(&self) -> FourDotsNonStackableMeritMemo {
        FourDotsNonStackableMeritMemo(self.0.as_memo())
    }
}

impl<'source> Deref for FourDotsNonStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FiveDotsNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> FiveDotsNonStackableMerit<'source> {
    pub fn as_memo(&self) -> FiveDotsNonStackableMeritMemo {
        FiveDotsNonStackableMeritMemo(self.0.as_memo())
    }
}

impl<'source> Deref for FiveDotsNonStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
