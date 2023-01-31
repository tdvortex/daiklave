use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::merits::merit::{template::MeritTemplateWithDotsMemo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum StackableMeritWithDotsMemo {
    Zero(MeritTemplateWithDotsMemo),
    One(MeritTemplateWithDotsMemo),
    Two(MeritTemplateWithDotsMemo),
    Three(MeritTemplateWithDotsMemo),
    Four(MeritTemplateWithDotsMemo),
    Five(MeritTemplateWithDotsMemo),
}

impl Deref for StackableMeritWithDotsMemo {
    type Target = MeritTemplateWithDotsMemo;

    fn deref(&self) -> &Self::Target {
        match self {
            StackableMeritWithDotsMemo::Zero(_) => todo!(),
            | StackableMeritWithDotsMemo::One(_) => todo!(),
            | StackableMeritWithDotsMemo::Two(_) => todo!(),
            | StackableMeritWithDotsMemo::Three(_) => todo!(),
            | StackableMeritWithDotsMemo::Four(_) => todo!(),
            | StackableMeritWithDotsMemo::Five(_) => todo!(),
        }
    }
}