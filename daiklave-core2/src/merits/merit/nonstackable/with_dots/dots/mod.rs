use std::ops::Deref;

use crate::merits::merit::template::MeritTemplateWithDots;

mod memo;
pub(crate) use memo::{FiveDotsNonStackableMeritMemo, FourDotsNonStackableMeritMemo, ThreeDotsNonStackableMeritMemo, TwoDotsNonStackableMeritMemo, OneDotNonStackableMeritMemo, ZeroDotsNonStackableMeritMemo};

pub(crate) struct ZeroDotsNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> Deref for ZeroDotsNonStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct OneDotNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> Deref for OneDotNonStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct TwoDotsNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> Deref for TwoDotsNonStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


pub(crate) struct ThreeDotsNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> Deref for ThreeDotsNonStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct FourDotsNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> Deref for FourDotsNonStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


pub(crate) struct FiveDotsNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> Deref for FiveDotsNonStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
