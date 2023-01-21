mod id;
pub use id::NonStackableMeritId;
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

use super::{prerequisite::MeritPrerequisite, MeritError};

/// A merit which is nonstackable, i.e. can only be purchased once per
/// character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NonStackableMerit(pub(crate) NonStackableMeritWithDotsMemo);

impl<'source> NonStackableMerit {
    /// Instantiates a new nonstackable merit from a template.
    pub fn new(
        template: NonStackableMeritTemplate,
        dots: u8,
    ) -> Result<(NonStackableMeritId, NonStackableMerit), MeritError> {
        let template_id = template.0;
        let with_dots = template.1.set_dots(dots)?;
        let dotted = match dots {
            0 => NonStackableMeritWithDotsMemo::Zero(ZeroDotsNonStackableMeritMemo(with_dots)),
            1 => NonStackableMeritWithDotsMemo::One(OneDotNonStackableMeritMemo(with_dots)),
            2 => NonStackableMeritWithDotsMemo::Two(TwoDotsNonStackableMeritMemo(with_dots)),
            3 => NonStackableMeritWithDotsMemo::Three(ThreeDotsNonStackableMeritMemo(with_dots)),
            4 => NonStackableMeritWithDotsMemo::Four(FourDotsNonStackableMeritMemo(with_dots)),
            5 => NonStackableMeritWithDotsMemo::Five(FiveDotsNonStackableMeritMemo(with_dots)),
            _ => {
                return Err(MeritError::InvalidDotRating);
            }
        };

        Ok((template_id, Self(dotted)))
    }

    pub(crate) fn as_ref(&'source self) -> NonStackableMeritView<'source> {
        NonStackableMeritView(self.0.as_ref())
    }

    /// Iterates over all prerequisites. If any prerequisite is met, the merit
    /// is purchasable. If an empty iterator, then it is also purchasable.
    pub fn prerequisites(&self) -> impl ExactSizeIterator<Item = MeritPrerequisite> + '_ {
        self.0.prerequisites()
    }
}
