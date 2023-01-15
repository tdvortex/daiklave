mod id;
pub(crate) use id::NonStackableMeritId;

use self::with_dots::{NonStackableMeritWithDotsMemo};

mod with_dots;

mod view;
pub(crate) use view::NonStackableMeritView;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonStackableMerit(NonStackableMeritWithDotsMemo);