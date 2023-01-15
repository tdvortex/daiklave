use crate::merits::merit::template::MeritTemplateWithDots;

mod memo;
pub(crate) use memo::{FiveDotsNonStackableMeritMemo, FourDotsNonStackableMeritMemo, ThreeDotsNonStackableMeritMemo, TwoDotsNonStackableMeritMemo, OneDotNonStackableMeritMemo, ZeroDotsNonStackableMeritMemo};

pub(crate) struct ZeroDotsNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> ZeroDotsNonStackableMerit<'source> {
    pub fn template_name(&self) -> &'source str {
        self.0.name()
    }
}

pub(crate) struct OneDotNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> OneDotNonStackableMerit<'source> {
    pub fn template_name(&self) -> &'source str {
        self.0.name()
    }
}

pub(crate) struct TwoDotsNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> TwoDotsNonStackableMerit<'source> {
    pub fn template_name(&self) -> &'source str {
        self.0.name()
    }
}

pub(crate) struct ThreeDotsNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> ThreeDotsNonStackableMerit<'source> {
    pub fn template_name(&self) -> &'source str {
        self.0.name()
    }
}

pub(crate) struct FourDotsNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> FourDotsNonStackableMerit<'source> {
    pub fn template_name(&self) -> &'source str {
        self.0.name()
    }
}

pub(crate) struct FiveDotsNonStackableMerit<'source>(MeritTemplateWithDots<'source>);

impl<'source> FiveDotsNonStackableMerit<'source> {
    pub fn template_name(&self) -> &'source str {
        self.0.name()
    }
}
