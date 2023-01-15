use crate::merits::merit::{template::MeritTemplateWithDotsMemo, stackable::StackableMeritTemplateId};

use super::{ZeroDotsStackableMerit, OneDotStackableMerit, TwoDotsStackableMerit, ThreeDotsStackableMerit, FourDotsStackableMerit, FiveDotsStackableMerit};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ZeroDotsStackableMeritMemo(StackableMeritTemplateId, MeritTemplateWithDotsMemo);

impl<'source> ZeroDotsStackableMeritMemo {
    pub fn as_ref(&'source self) -> ZeroDotsStackableMerit<'source> {
        ZeroDotsStackableMerit(self.0, self.1.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OneDotStackableMeritMemo(StackableMeritTemplateId, MeritTemplateWithDotsMemo);

impl<'source> OneDotStackableMeritMemo {
    pub fn as_ref(&'source self) -> OneDotStackableMerit<'source> {
        OneDotStackableMerit(self.0, self.1.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TwoDotsStackableMeritMemo(StackableMeritTemplateId, MeritTemplateWithDotsMemo);

impl<'source> TwoDotsStackableMeritMemo {
    pub fn as_ref(&'source self) -> TwoDotsStackableMerit<'source> {
        TwoDotsStackableMerit(self.0, self.1.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ThreeDotsStackableMeritMemo(StackableMeritTemplateId, MeritTemplateWithDotsMemo);

impl<'source> ThreeDotsStackableMeritMemo {
    pub fn as_ref(&'source self) -> ThreeDotsStackableMerit<'source> {
        ThreeDotsStackableMerit(self.0, self.1.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FourDotsStackableMeritMemo(StackableMeritTemplateId, MeritTemplateWithDotsMemo);


impl<'source> FourDotsStackableMeritMemo {
    pub fn as_ref(&'source self) -> FourDotsStackableMerit<'source> {
        FourDotsStackableMerit(self.0, self.1.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FiveDotsStackableMeritMemo(StackableMeritTemplateId, MeritTemplateWithDotsMemo);

impl<'source> FiveDotsStackableMeritMemo {
    pub fn as_ref(&'source self) -> FiveDotsStackableMerit<'source> {
        FiveDotsStackableMerit(self.0, self.1.as_ref())
    }
}