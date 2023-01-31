use crate::{
    book_reference::BookReference,
    merits::merit::{MeritPrerequisite, MeritType, template::MeritTemplateWithDotsMemo},
};

use super::{with_dots::StackableMeritWithDots};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StackableMeritView<'source>(&'source MeritTemplateWithDotsMemo);