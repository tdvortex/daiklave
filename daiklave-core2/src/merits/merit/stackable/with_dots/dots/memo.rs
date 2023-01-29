use serde::{Deserialize, Serialize};

use crate::merits::merit::template::MeritTemplateWithDotsMemo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ZeroDotsStackableMeritMemo(pub MeritTemplateWithDotsMemo);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct OneDotStackableMeritMemo(pub MeritTemplateWithDotsMemo);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TwoDotsStackableMeritMemo(pub MeritTemplateWithDotsMemo);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ThreeDotsStackableMeritMemo(pub MeritTemplateWithDotsMemo);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FourDotsStackableMeritMemo(pub MeritTemplateWithDotsMemo);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FiveDotsStackableMeritMemo(pub MeritTemplateWithDotsMemo);
