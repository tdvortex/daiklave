mod name;
pub use name::StackableMeritTemplateName;
mod template;
pub use template::StackableMeritTemplate;

mod view;
use serde::{Deserialize, Serialize};
pub(crate) use view::StackableMeritView;

pub(crate) use with_dots::{
    FiveDotsStackableMeritMemo, FourDotsStackableMeritMemo, OneDotStackableMeritMemo,
    StackableMeritWithDotsMemo, ThreeDotsStackableMeritMemo, TwoDotsStackableMeritMemo,
    ZeroDotsStackableMeritMemo,
};

use super::{MeritError, MeritPrerequisite};

mod with_dots;

/// A merit which may be purchased more than once, along with a detail for this
/// specific instance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StackableMerit {
    pub(crate) dotted: StackableMeritWithDotsMemo,
}

impl<'source> StackableMerit {
    /// Creates a new Stackable merit from a template. Errors if an invalid
    /// number of dots is specified.
    pub fn new(
        template: StackableMeritTemplate,
        dots: u8,
        detail: String,
    ) -> Result<Self, MeritError> {
        let template_id = template.0;
        let with_dots = template.0.set_dots(dots)?;
        let dotted = match dots {
            0 => StackableMeritWithDotsMemo::Zero(ZeroDotsStackableMeritMemo(with_dots)),
            1 => StackableMeritWithDotsMemo::One(OneDotStackableMeritMemo(with_dots)),
            2 => StackableMeritWithDotsMemo::Two(TwoDotsStackableMeritMemo(with_dots)),
            3 => StackableMeritWithDotsMemo::Three(ThreeDotsStackableMeritMemo(with_dots)),
            4 => StackableMeritWithDotsMemo::Four(FourDotsStackableMeritMemo(with_dots)),
            5 => StackableMeritWithDotsMemo::Five(FiveDotsStackableMeritMemo(with_dots)),
            _ => {
                return Err(MeritError::InvalidDotRating);
            }
        };

        Ok(Self { dotted })
    }

    /// Iterates over all prerequisites. If any prerequisite is met, the merit
    /// is purchasable. If an empty iterator, then it is also purchasable.
    pub fn prerequisites(&self) -> impl ExactSizeIterator<Item = MeritPrerequisite> + '_ {
        self.dotted.prerequisites()
    }
}
