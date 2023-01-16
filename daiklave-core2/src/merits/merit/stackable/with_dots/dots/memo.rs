use serde::{Deserialize, Serialize};

use crate::merits::merit::{
    stackable::StackableMeritTemplateId, template::MeritTemplateWithDotsMemo,
};

use super::{
    FiveDotsStackableMerit, FourDotsStackableMerit, OneDotStackableMerit, ThreeDotsStackableMerit,
    TwoDotsStackableMerit, ZeroDotsStackableMerit,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ZeroDotsStackableMeritMemo(
    pub StackableMeritTemplateId,
    pub MeritTemplateWithDotsMemo,
);

impl<'source> ZeroDotsStackableMeritMemo {
    pub fn as_ref(&'source self) -> ZeroDotsStackableMerit<'source> {
        ZeroDotsStackableMerit(self.0, self.1.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct OneDotStackableMeritMemo(
    pub StackableMeritTemplateId,
    pub MeritTemplateWithDotsMemo,
);

impl<'source> OneDotStackableMeritMemo {
    pub fn as_ref(&'source self) -> OneDotStackableMerit<'source> {
        OneDotStackableMerit(self.0, self.1.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TwoDotsStackableMeritMemo(
    pub StackableMeritTemplateId,
    pub MeritTemplateWithDotsMemo,
);

impl<'source> TwoDotsStackableMeritMemo {
    pub fn as_ref(&'source self) -> TwoDotsStackableMerit<'source> {
        TwoDotsStackableMerit(self.0, self.1.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ThreeDotsStackableMeritMemo(
    pub StackableMeritTemplateId,
    pub MeritTemplateWithDotsMemo,
);

impl<'source> ThreeDotsStackableMeritMemo {
    pub fn as_ref(&'source self) -> ThreeDotsStackableMerit<'source> {
        ThreeDotsStackableMerit(self.0, self.1.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FourDotsStackableMeritMemo(
    pub StackableMeritTemplateId,
    pub MeritTemplateWithDotsMemo,
);

impl<'source> FourDotsStackableMeritMemo {
    pub fn as_ref(&'source self) -> FourDotsStackableMerit<'source> {
        FourDotsStackableMerit(self.0, self.1.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FiveDotsStackableMeritMemo(
    pub StackableMeritTemplateId,
    pub MeritTemplateWithDotsMemo,
);

impl<'source> FiveDotsStackableMeritMemo {
    pub fn as_ref(&'source self) -> FiveDotsStackableMerit<'source> {
        FiveDotsStackableMerit(self.0, self.1.as_ref())
    }
}
