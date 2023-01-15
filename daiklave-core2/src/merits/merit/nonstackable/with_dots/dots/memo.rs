use serde::{Serialize, Deserialize};

use crate::merits::merit::template::MeritTemplateWithDotsMemo;

use super::{ZeroDotsNonStackableMerit, OneDotNonStackableMerit, TwoDotsNonStackableMerit, ThreeDotsNonStackableMerit, FourDotsNonStackableMerit, FiveDotsNonStackableMerit};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ZeroDotsNonStackableMeritMemo(pub MeritTemplateWithDotsMemo);

impl<'source> ZeroDotsNonStackableMeritMemo {
    pub fn as_ref(&'source self) -> ZeroDotsNonStackableMerit<'source> {
        ZeroDotsNonStackableMerit(self.0.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct OneDotNonStackableMeritMemo(pub MeritTemplateWithDotsMemo);

impl<'source> OneDotNonStackableMeritMemo {
    pub fn as_ref(&'source self) -> OneDotNonStackableMerit<'source> {
        OneDotNonStackableMerit(self.0.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TwoDotsNonStackableMeritMemo(pub MeritTemplateWithDotsMemo);

impl<'source> TwoDotsNonStackableMeritMemo {
    pub fn as_ref(&'source self) -> TwoDotsNonStackableMerit<'source> {
        TwoDotsNonStackableMerit(self.0.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ThreeDotsNonStackableMeritMemo(pub MeritTemplateWithDotsMemo);

impl<'source> ThreeDotsNonStackableMeritMemo {
    pub fn as_ref(&'source self) -> ThreeDotsNonStackableMerit<'source> {
        ThreeDotsNonStackableMerit(self.0.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FourDotsNonStackableMeritMemo(pub MeritTemplateWithDotsMemo);

impl<'source> FourDotsNonStackableMeritMemo {
    pub fn as_ref(&'source self) -> FourDotsNonStackableMerit<'source> {
        FourDotsNonStackableMerit(self.0.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FiveDotsNonStackableMeritMemo(pub MeritTemplateWithDotsMemo);

impl<'source> FiveDotsNonStackableMeritMemo {
    pub fn as_ref(&'source self) -> FiveDotsNonStackableMerit<'source> {
        FiveDotsNonStackableMerit(self.0.as_ref())
    }
}