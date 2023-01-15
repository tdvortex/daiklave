use std::ops::Deref;

use crate::merits::merit::{template::{MeritTemplateWithDots, MeritTemplateId}, stackable::StackableMeritTemplateId};

mod memo;
pub(crate) use memo::{ZeroDotsStackableMeritMemo, OneDotStackableMeritMemo, TwoDotsStackableMeritMemo, ThreeDotsStackableMeritMemo, FourDotsStackableMeritMemo, FiveDotsStackableMeritMemo};

pub(crate) struct ZeroDotsStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> Deref for ZeroDotsStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'source> ZeroDotsStackableMerit<'source> {
    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::StackableMerit(self.0)
    }
}

pub(crate) struct OneDotStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> Deref for OneDotStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'source> OneDotStackableMerit<'source> {
    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::StackableMerit(self.0)
    }
}

pub(crate) struct TwoDotsStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> Deref for TwoDotsStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'source> TwoDotsStackableMerit<'source> {
    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::StackableMerit(self.0)
    }
}

pub(crate) struct ThreeDotsStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> Deref for ThreeDotsStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'source> ThreeDotsStackableMerit<'source> {
    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::StackableMerit(self.0)
    }
}

pub(crate) struct FourDotsStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> Deref for FourDotsStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'source> FourDotsStackableMerit<'source> {
    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::StackableMerit(self.0)
    }
}

pub(crate) struct FiveDotsStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> Deref for FiveDotsStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'source> FiveDotsStackableMerit<'source> {
    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::StackableMerit(self.0)
    }
}