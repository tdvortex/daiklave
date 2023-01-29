use crate::merits::merit::{template::MeritTemplate, add::AddNonStackableMerit, MeritError};

use super::{NonStackableMerit, NonStackableMeritName, NonStackableMeritWithDotsMemo};

pub struct NonStackableMeritTemplate(pub(crate) NonStackableMeritName, pub(crate) MeritTemplate);

impl NonStackableMeritTemplate {
    pub fn with_dots(self, dots: u8) -> Result<AddNonStackableMerit, MeritError> {
        let template_name = self.0;
        let with_dots = self.1.set_dots(dots)?;
        let dotted = match dots {
            0 => NonStackableMeritWithDotsMemo::Zero(with_dots),
            1 => NonStackableMeritWithDotsMemo::One(with_dots),
            2 => NonStackableMeritWithDotsMemo::Two(with_dots),
            3 => NonStackableMeritWithDotsMemo::Three(with_dots),
            4 => NonStackableMeritWithDotsMemo::Four(with_dots),
            5 => NonStackableMeritWithDotsMemo::Five(with_dots),
            _ => {
                return Err(MeritError::InvalidDotRating);
            }
        };

        Ok(
            AddNonStackableMerit {
                name: template_name.into(),
                merit: NonStackableMerit(dotted),
            }
        )
    }
}