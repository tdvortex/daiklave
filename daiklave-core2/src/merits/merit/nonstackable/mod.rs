use serde::{Deserialize, Serialize};

pub(crate) use with_dots::NonStackableMeritWithDotsMemo;

mod name;
pub use name::NonStackableMeritName;
mod template;
pub use template::NonStackableMeritTemplate;

mod with_dots;

mod view;
pub(crate) use view::NonStackableMeritView;

use super::{prerequisite::MeritPrerequisite};

/// A merit which is nonstackable, i.e. can only be purchased once per
/// character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NonStackableMerit(pub(crate) NonStackableMeritWithDotsMemo);

impl<'source> NonStackableMerit {
    /// Iterates over all prerequisites. If any prerequisite is met, the merit
    /// is purchasable. If an empty iterator, then it is also purchasable.
    pub fn prerequisites(&self) -> impl ExactSizeIterator<Item = MeritPrerequisite> + '_ {
        (*self.0).prerequisites.iter().copied()
    }
}
