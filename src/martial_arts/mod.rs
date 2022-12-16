use crate::{charms::MartialArtsCharm, data_source::DataSource, id::Id, abilities::{AbilityRating, NonZeroAbility, Ability, AbilityName}};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub(crate) struct MartialArtsStyle {
    id: Id,
    name: String,
    description: Option<String>,
    data_source: DataSource,
}

impl PartialEq for MartialArtsStyle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl MartialArtsStyle {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MartialArtistDetails {
    style: MartialArtsStyle,
    rating: AbilityRating,
    charms: Vec<MartialArtsCharm>,
}

impl MartialArtistDetails {
    pub fn style(&self) -> &MartialArtsStyle {
        &self.style
    }

    pub fn as_ability(&self) -> Ability {
        Ability {
            name: AbilityName::MartialArts(self.style.name()),
            rating: &self.rating
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct MartialArtistTraits(pub Vec<MartialArtistDetails>);

impl MartialArtistTraits {
    pub fn add_style(&mut self, style: MartialArtsStyle, dots: u8) -> Result<()> {
        if self.0.iter().any(|details| details.style == style) {
            Err(eyre!("Style {} already known", style.name))
        } else {
            let rating = if dots == 0 {
                AbilityRating::Zero
            } else {
                AbilityRating::NonZero(NonZeroAbility{dots, specialties: Vec::new()})
            };

            self.0.push(MartialArtistDetails {
                style,
                rating,
                charms: Default::default(),
            });
            self.0
                .sort_by(|a, b| a.style.name.as_str().cmp(b.style.name.as_str()));
            Ok(())
        }
    }
}
