mod id;
pub(crate) use id::StackableMeritTemplateId;
pub use id::{StackableMeritId};

mod view;
pub(crate) use view::StackableMeritView;

use self::with_dots::StackableMeritWithDotsMemo;

mod with_dots;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackableMerit {
    detail: String,
    dotted: StackableMeritWithDotsMemo,
}

impl<'source> StackableMerit {
    pub(crate) fn as_ref(&'source self) -> StackableMeritView<'source> {
        StackableMeritView {
            detail: self.detail.as_str(),
            dotted: self.dotted.as_ref(),
        }
    }
}