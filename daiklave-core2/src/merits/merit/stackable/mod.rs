mod name;
mod template;
mod view;
mod with_dots;
pub use name::StackableMeritTemplateName;
pub use template::StackableMeritTemplate;
pub(crate) use view::StackableMeritView;


use serde::{Deserialize, Serialize};

use self::with_dots::StackableMeritWithDotsMemo;

use super::{MeritPrerequisite};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StackableMerit {
    pub(crate) dotted: StackableMeritWithDotsMemo,
}

impl<'source> StackableMerit {
    /// Iterates over all prerequisites. If any prerequisite is met, the merit
    /// is purchasable. If an empty iterator, then it is also purchasable.
    pub fn prerequisites(&self) -> impl ExactSizeIterator<Item = MeritPrerequisite> + '_ {
        self.dotted.prerequisites()
    }
}
