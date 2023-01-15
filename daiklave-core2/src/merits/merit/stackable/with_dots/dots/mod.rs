use std::ops::Deref;

use crate::merits::merit::{template::{MeritTemplateWithDots, MeritTemplateId}, stackable::StackableMeritTemplateId};

mod memo;
pub(crate) use memo::{ZeroDotsStackableMeritMemo, OneDotStackableMeritMemo, TwoDotsStackableMeritMemo, ThreeDotsStackableMeritMemo, FourDotsStackableMeritMemo, FiveDotsStackableMeritMemo};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ZeroDotsStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> Deref for ZeroDotsStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'source> ZeroDotsStackableMerit<'source> {
    pub fn as_memo(&self) -> ZeroDotsStackableMeritMemo {
        ZeroDotsStackableMeritMemo(self.0, self.1.as_memo())
    }

    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::Stackable(self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OneDotStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> Deref for OneDotStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'source> OneDotStackableMerit<'source> {
    pub fn as_memo(&self) -> OneDotStackableMeritMemo {
        OneDotStackableMeritMemo(self.0, self.1.as_memo())
    }

    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::Stackable(self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TwoDotsStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> Deref for TwoDotsStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'source> TwoDotsStackableMerit<'source> {
    pub fn as_memo(&self) -> TwoDotsStackableMeritMemo {
        TwoDotsStackableMeritMemo(self.0, self.1.as_memo())
    }

    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::Stackable(self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ThreeDotsStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> Deref for ThreeDotsStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'source> ThreeDotsStackableMerit<'source> {
    pub fn as_memo(&self) -> ThreeDotsStackableMeritMemo {
        ThreeDotsStackableMeritMemo(self.0, self.1.as_memo())
    }

    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::Stackable(self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FourDotsStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> Deref for FourDotsStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'source> FourDotsStackableMerit<'source> {
    pub fn as_memo(&self) -> FourDotsStackableMeritMemo {
        FourDotsStackableMeritMemo(self.0, self.1.as_memo())
    }
    
    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::Stackable(self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FiveDotsStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> Deref for FiveDotsStackableMerit<'source> {
    type Target = MeritTemplateWithDots<'source>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'source> FiveDotsStackableMerit<'source> {
    pub fn as_memo(&self) -> FiveDotsStackableMeritMemo {
        FiveDotsStackableMeritMemo(self.0, self.1.as_memo())
    }

    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::Stackable(self.0)
    }
}