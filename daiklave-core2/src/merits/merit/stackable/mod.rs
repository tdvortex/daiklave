mod id;
pub(crate) use id::StackableMeritTemplateId;
pub use id::{StackableMeritId};

mod template;
pub use template::StackableMeritTemplate;

mod view;
use serde::{Serialize, Deserialize};
pub(crate) use view::StackableMeritView;

pub(crate) use with_dots::{StackableMeritWithDotsMemo, ZeroDotsStackableMeritMemo, OneDotStackableMeritMemo, TwoDotsStackableMeritMemo, ThreeDotsStackableMeritMemo, FourDotsStackableMeritMemo, FiveDotsStackableMeritMemo};

use super::MeritError;

mod with_dots;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StackableMerit {
    pub(crate) detail: String,
    pub(crate) dotted: StackableMeritWithDotsMemo,
}

impl<'source> StackableMerit {
    pub fn new(template: StackableMeritTemplate, dots: u8, detail: String) -> Result<Self, MeritError> {
        let template_id = template.0;
        let with_dots = template.1.set_dots(dots)?;
        let dotted = match dots {
            0 => StackableMeritWithDotsMemo::Zero(ZeroDotsStackableMeritMemo(template_id, with_dots)),
            1 => StackableMeritWithDotsMemo::One(OneDotStackableMeritMemo(template_id, with_dots)),
            2 => StackableMeritWithDotsMemo::Two(TwoDotsStackableMeritMemo(template_id, with_dots)),
            3 => StackableMeritWithDotsMemo::Three(ThreeDotsStackableMeritMemo(template_id, with_dots)),
            4 => StackableMeritWithDotsMemo::Four(FourDotsStackableMeritMemo(template_id, with_dots)),
            5 => StackableMeritWithDotsMemo::Five(FiveDotsStackableMeritMemo(template_id, with_dots)),
            _ => {return Err(MeritError::InvalidDotRating);}
        };

        Ok(Self {
            detail,
            dotted,
        })
    }

    pub(crate) fn as_ref(&'source self) -> StackableMeritView<'source> {
        StackableMeritView {
            detail: self.detail.as_str(),
            dotted: self.dotted.as_ref(),
        }
    }
}