use crate::merits::merit::template::MeritTemplateId;

use super::with_dots::StackableMeritWithDots;

pub(crate) struct StackableMeritView<'source> {
    pub detail: &'source str,
    pub dotted: StackableMeritWithDots<'source>,
}

impl<'source> StackableMeritView<'source> {
    pub fn template_id(&self) -> MeritTemplateId {
        self.dotted.template_id()
    }

    pub fn template_name(&self) -> &'source str {
        self.dotted.template_name()
    }
}