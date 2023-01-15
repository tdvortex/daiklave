mod id;
pub(crate) use id::NonStackableMeritId;
use serde::{Deserialize, Serialize};

pub(crate) use with_dots::{
    FiveDotsNonStackableMeritMemo, FourDotsNonStackableMeritMemo, NonStackableMeritWithDotsMemo,
    OneDotNonStackableMeritMemo, ThreeDotsNonStackableMeritMemo, TwoDotsNonStackableMeritMemo,
    ZeroDotsNonStackableMeritMemo,
};

mod template;
pub use template::NonStackableMeritTemplate;

mod with_dots;

mod view;
pub(crate) use view::NonStackableMeritView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NonStackableMerit(pub(crate) NonStackableMeritWithDotsMemo);

impl<'source> NonStackableMerit {
    pub(crate) fn as_ref(&'source self) -> NonStackableMeritView<'source> {
        NonStackableMeritView(self.0.as_ref())
    }
}
