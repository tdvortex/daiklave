use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::merits::merit::{template::MeritTemplateWithDotsMemo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum NonStackableMeritWithDotsMemo {
    Zero(MeritTemplateWithDotsMemo),
    One(MeritTemplateWithDotsMemo),
    Two(MeritTemplateWithDotsMemo),
    Three(MeritTemplateWithDotsMemo),
    Four(MeritTemplateWithDotsMemo),
    Five(MeritTemplateWithDotsMemo),
}

impl Deref for NonStackableMeritWithDotsMemo {
    type Target = MeritTemplateWithDotsMemo;

    fn deref(&self) -> &Self::Target {
        match &self {
            NonStackableMeritWithDotsMemo::Zero(with_dots)
            | NonStackableMeritWithDotsMemo::One(with_dots)
            | NonStackableMeritWithDotsMemo::Two(with_dots)
            | NonStackableMeritWithDotsMemo::Three(with_dots)
            | NonStackableMeritWithDotsMemo::Four(with_dots)
            | NonStackableMeritWithDotsMemo::Five(with_dots) => with_dots,
        }
    }
}