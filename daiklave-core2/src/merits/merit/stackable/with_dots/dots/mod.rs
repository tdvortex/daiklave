use crate::merits::merit::{template::{MeritTemplateWithDots, MeritTemplateId}, stackable::StackableMeritTemplateId};

mod memo;
pub(crate) use memo::{ZeroDotsStackableMeritMemo, OneDotStackableMeritMemo, TwoDotsStackableMeritMemo, ThreeDotsStackableMeritMemo, FourDotsStackableMeritMemo, FiveDotsStackableMeritMemo};

pub(crate) struct ZeroDotsStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> ZeroDotsStackableMerit<'source> {
    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::StackableMerit(self.0)
    }

    pub fn template_name(&self) -> &'source str {
        self.1.name()
    }
}

pub(crate) struct OneDotStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> OneDotStackableMerit<'source> {
    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::StackableMerit(self.0)
    }

    pub fn template_name(&self) -> &'source str {
        self.1.name()
    }
}

pub(crate) struct TwoDotsStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> TwoDotsStackableMerit<'source> {
    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::StackableMerit(self.0)
    }

    pub fn template_name(&self) -> &'source str {
        self.1.name()
    }
}

pub(crate) struct ThreeDotsStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> ThreeDotsStackableMerit<'source> {
    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::StackableMerit(self.0)
    }

    pub fn template_name(&self) -> &'source str {
        self.1.name()
    }
}

pub(crate) struct FourDotsStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> FourDotsStackableMerit<'source> {
    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::StackableMerit(self.0)
    }

    pub fn template_name(&self) -> &'source str {
        self.1.name()
    }
}

pub(crate) struct FiveDotsStackableMerit<'source>(StackableMeritTemplateId, MeritTemplateWithDots<'source>);

impl<'source> FiveDotsStackableMerit<'source> {
    pub fn template_id(&self) -> MeritTemplateId {
        MeritTemplateId::StackableMerit(self.0)
    }

    pub fn template_name(&self) -> &'source str {
        self.1.name()
    }
}