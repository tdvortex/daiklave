mod tables;
mod update;
use crate::abilities::{AbilityRating, NonZeroAbility, Ability, AbilityName};
use eyre::{eyre, Result};
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub(crate) struct CraftAbilities(Vec<(String, AbilityRating)>);

impl CraftAbilities {
    pub(crate) fn get_rating(&self, focus: &str) -> Option<&AbilityRating> {
        self.0.iter().find_map(|(known_focus, rating)| if known_focus.as_str() == focus {
            Some(rating)
        } else {
            None
        })
    }

    fn get_rating_mut(&mut self, focus: &str) -> Option<&mut AbilityRating> {
        self.0.iter_mut().find_map(|(known_focus, rating)| if known_focus.as_str() == focus {
            Some(rating)
        } else {
            None
        })
    }

    fn add_focus(&mut self, focus: String, dots: u8) -> Result<()> {
        if dots == 0 {
            Err(eyre!("Cannot add a Craft focus with zero dots"))
        } else {
            self.0.push((focus, AbilityRating::NonZero(NonZeroAbility{dots, specialties: Vec::new()})));
            self.0.sort_by(|(a_focus, _), (b_focus, _)| a_focus.as_str().cmp(b_focus.as_str()));
            self.0.dedup_by(|(a_focus, _), (b_focus, _)| a_focus.as_str() == b_focus.as_str());
            Ok(())
        }        
    }

    pub fn iter(&self) -> impl Iterator<Item = Ability> {
        self.0.iter().map(|(focus_string, rating)| Ability { name: AbilityName::Craft(focus_string.as_str()), rating})
    }

    pub fn set_dots(&mut self, focus: &str, dots: u8) {
        if dots == 0 {
            self.0.iter().enumerate().find_map(|(index, (known_focus, _))| if known_focus.as_str() == focus {
                Some(index)
            } else {
                None
            }).map(|delete_index| self.0.remove(delete_index));
        } else {
            if let Some(rating) = self.get_rating_mut(focus) {
                rating.set_dots(dots);
            } else {
                self.add_focus(focus.to_owned(), dots).unwrap()
            }
        }
    }

    pub fn add_specialty(&mut self, focus: &str, specialty: String) -> Result<()> {
        self.get_rating_mut(focus).ok_or_else(|| eyre!("Zero-rated abilities cannot have specialties"))?.add_specialty(specialty)
    }

    pub fn remove_specialty(&mut self, focus: &str, specialty: &str) -> Result<()> {
        self.get_rating_mut(focus).ok_or_else(|| eyre!("Zero-rated abilities cannot have specialties"))?.remove_specialty(specialty)
    }

}

