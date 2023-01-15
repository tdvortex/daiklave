mod id;
pub(crate) use id::StackableMeritTemplateId;
pub use id::{StackableMeritId};

mod template;
pub use template::StackableMeritTemplate;

mod view;
use serde::{Serialize, Deserialize};
pub(crate) use view::StackableMeritView;

pub(crate) use with_dots::{StackableMeritWithDotsMemo, ZeroDotsStackableMeritMemo, OneDotStackableMeritMemo, TwoDotsStackableMeritMemo, ThreeDotsStackableMeritMemo, FourDotsStackableMeritMemo, FiveDotsStackableMeritMemo};

mod with_dots;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StackableMerit {
    pub(crate) detail: String,
    pub(crate) dotted: StackableMeritWithDotsMemo,
}

impl<'source> StackableMerit {
    pub(crate) fn as_ref(&'source self) -> StackableMeritView<'source> {
        StackableMeritView {
            detail: self.detail.as_str(),
            dotted: self.dotted.as_ref(),
        }
    }
}