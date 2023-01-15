mod id;
pub(crate) use id::NonStackableMeritId;
use serde::{Serialize, Deserialize};

use self::with_dots::{NonStackableMeritWithDotsMemo};

mod with_dots;

mod view;
pub(crate) use view::NonStackableMeritView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NonStackableMerit(NonStackableMeritWithDotsMemo);

impl<'source> NonStackableMerit {
    pub(crate) fn as_ref(&'source self) -> NonStackableMeritView<'source> {
        NonStackableMeritView(self.0.as_ref())
    }
}