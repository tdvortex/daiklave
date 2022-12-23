pub mod diff;
use std::collections::HashSet;

pub use crate::charms::{MartialArtsCharm, MartialArtsCharmBuilder};
use crate::{
    abilities::{Ability, AbilityName, AbilityRating, NonZeroAbility},
    data_source::{BookReference, DataSource},
    id::{CharacterId, MartialArtsCharmId, MartialArtsStyleId},
};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct MartialArtsStyle {
    id: MartialArtsStyleId,
    name: String,
    description: String,
    data_source: DataSource,
}

impl PartialEq for MartialArtsStyle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl MartialArtsStyle {
    pub fn from_book(
        id: MartialArtsStyleId,
        book_title: String,
        page_number: i16,
    ) -> MartialArtsStyleBuilder {
        MartialArtsStyleBuilder {
            id,
            data_source: DataSource::Book(BookReference {
                book_title,
                page_number,
            }),
            name: None,
            description: None,
        }
    }

    pub fn custom(id: MartialArtsStyleId, creator_id: CharacterId) -> MartialArtsStyleBuilder {
        MartialArtsStyleBuilder {
            id,
            data_source: DataSource::Custom(creator_id),
            name: None,
            description: None,
        }
    }

    pub fn id(&self) -> MartialArtsStyleId {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn data_source(&self) -> &DataSource {
        &self.data_source
    }
}

pub struct MartialArtsStyleBuilder {
    id: MartialArtsStyleId,
    data_source: DataSource,
    name: Option<String>,
    description: Option<String>,
}

impl MartialArtsStyleBuilder {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn build(self) -> Result<MartialArtsStyle> {
        Ok(MartialArtsStyle {
            id: self.id,
            name: self
                .name
                .ok_or_else(|| eyre!("Martial Arts styles must be named"))?,
            description: self
                .description
                .ok_or_else(|| eyre!("Martial Arts styles must be have a description"))?,
            data_source: self.data_source,
        })
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
            rating: &self.rating,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct MartialArtistTraits(Vec<MartialArtistDetails>);

impl MartialArtistTraits {
    pub fn get_ability(&self, style_id: MartialArtsStyleId) -> Option<Ability> {
        self.0
            .iter()
            .find(|&details| details.style().id() == style_id)
            .map(|d| d.as_ability())
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&MartialArtsStyle, Ability, &Vec<MartialArtsCharm>)> {
        self.0
            .iter()
            .map(|details| (details.style(), details.as_ability(), &details.charms))
    }

    pub fn add_style(&mut self, style: MartialArtsStyle, dots: u8) -> Result<()> {
        if self.0.iter().any(|details| details.style == style) {
            Err(eyre!("Style {} already known", style.name))
        } else {
            let rating = if dots == 0 {
                AbilityRating::Zero
            } else {
                AbilityRating::NonZero(NonZeroAbility {
                    dots,
                    specialties: Vec::new(),
                })
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

    fn get_rating_mut(&mut self, style_id: MartialArtsStyleId) -> Result<&mut AbilityRating> {
        Ok(&mut self
            .0
            .iter_mut()
            .find(|details| details.style().id() == style_id)
            .ok_or_else(|| eyre!("Martial Arts style {} not found", **style_id))?
            .rating)
    }

    pub fn set_dots(&mut self, style_id: MartialArtsStyleId, dots: u8) -> Result<()> {
        self.get_rating_mut(style_id)?.set_dots(dots);
        Ok(())
    }

    pub fn add_specialty(&mut self, style_id: MartialArtsStyleId, specialty: String) -> Result<()> {
        self.get_rating_mut(style_id)?.add_specialty(specialty)
    }

    pub fn remove_specialty(
        &mut self,
        style_id: MartialArtsStyleId,
        specialty: &str,
    ) -> Result<()> {
        self.get_rating_mut(style_id)?.remove_specialty(specialty)
    }

    pub fn add_charm(&mut self, charm: MartialArtsCharm) -> Result<()> {
        // Do NOT check Essence here--needs to be done at a level above ExaltType
        // DO check style, ability dots, and charm prerequisites
        let style_id = charm.style_id();

        let ability = self
            .get_ability(style_id)
            .ok_or_else(|| eyre!("Martial Arts style {} not found", **style_id))?;
        if ability.dots() < charm.martial_arts_requirement() {
            return Err(eyre!(
                "Insufficient Martial Arts {} dots, need {} but only have {}",
                ability.name().subskill().unwrap(),
                charm.martial_arts_requirement(),
                ability.dots()
            ));
        }

        let charms = &mut self
            .0
            .iter_mut()
            .find(|details| details.style().id() == style_id)
            .ok_or_else(|| eyre!("Martial Arts style {} not found", **style_id))?
            .charms;

        let known_charm_ids = charms
            .iter()
            .map(|known_charm| known_charm.id())
            .collect::<HashSet<MartialArtsCharmId>>();
        for id in charm.prerequisite_charm_ids().iter() {
            if !known_charm_ids.contains(id) {
                return Err(eyre!("Not all Charm prerequisites met"));
            }
        }

        charms.push(charm);
        charms.sort_by(|a, b| a.name().cmp(b.name()));
        charms.dedup();
        Ok(())
    }
}
