use crate::{
    abilities::{
        Abilities, AbilitiesVanilla, AbilityError, AbilityNameQualifiedMutation, AbilityNameVanilla,
    },
    character::mutation::{AddSpecialty, RemoveSpecialty, SetAbility},
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    pub(crate) fn vanilla_abilities(&'view self) -> &'view AbilitiesVanilla<'source> {
        &self.abilities
    }

    /// Get read-only access to a character’s Abilities.
    pub fn abilities(&'view self) -> Abilities<'view, 'source> {
        Abilities(self)
    }

    /// Sets the dot rating for a non-Craft, non-Martial Arts ability.
    pub fn set_vanilla_ability_dots(
        &mut self,
        ability_name: AbilityNameVanilla,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if dots > 5 {
            return Err(CharacterMutationError::AbilityError(
                AbilityError::InvalidRating,
            ));
        }

        let old_dots = self.abilities().get_vanilla(ability_name.into()).dots();
        self.abilities.get_mut(ability_name).set_dots(dots)?;

        if old_dots > dots {
            let sorcery_removed = self.correct_sorcery_level();

            let ma_style_removed = if ability_name == AbilityNameVanilla::Brawl && dots == 0 {
                let mut ma_style_removed = false;
                for style_name in self.martial_arts().iter() {
                    if self.remove_martial_arts_style(style_name).is_ok() {
                        ma_style_removed = true;
                    }
                }
                ma_style_removed
            } else {
                false
            };

            self.correct_merits();
            let solar_charm_removed = self.correct_solar_charms(&[]);

            // Evocations don't depend on abilities, but they may depend on
            // Spells, Martial Arts, or Solar Charms
            if sorcery_removed || ma_style_removed || solar_charm_removed {
                self.correct_evocations(&[]);
            }
        }

        Ok(self)
    }

    /// Set an ability's dots to a specific dot value. If this sets the ability
    /// to 0 dots, will erase all specialties. If Occult is lowered, may cause
    /// Sorcery circles to be dropped. If Brawl is lowered to 0, will cause all
    /// Martial Arts styles (and their associated Charm) to be dropped. May
    /// cause Solar Charms to be dropped, cascading to all dependent Charms.
    pub fn set_ability_dots(
        &mut self,
        set_ability: &'source SetAbility,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &set_ability.name {
            AbilityNameQualifiedMutation::Vanilla(vanilla) => {
                self.set_vanilla_ability_dots(*vanilla, set_ability.dots)
            }
            AbilityNameQualifiedMutation::Craft(focus) => {
                self.set_craft_dots(focus, set_ability.dots)
            }
            AbilityNameQualifiedMutation::MartialArts(style) => {
                self.set_martial_arts_dots(style, set_ability.dots)
            }
        }
    }

    /// Adds a specialty to an ability.
    pub fn add_specialty(
        &mut self,
        add_specialty: &'source AddSpecialty,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &add_specialty.ability_name {
            AbilityNameQualifiedMutation::Vanilla(vanilla) => {
                self.add_vanilla_specialty(*vanilla, &add_specialty.specialty)
            }
            AbilityNameQualifiedMutation::Craft(focus) => {
                self.add_craft_specialty(focus, &add_specialty.specialty)
            }
            AbilityNameQualifiedMutation::MartialArts(style) => {
                self.add_martial_arts_specialty(style, &add_specialty.specialty)
            }
        }
    }

    /// Adds a specialty to a non-Craft, non-Martial Arts ability.
    pub fn add_vanilla_specialty(
        &mut self,
        ability_name: AbilityNameVanilla,
        specialty: &'source str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.abilities
            .get_mut(ability_name)
            .add_specialty(specialty)?;
        Ok(self)
    }

    /// Removes a specialty from an ability.
    pub fn remove_specialty(
        &mut self,
        remove_specialty: &RemoveSpecialty,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &remove_specialty.ability_name {
            AbilityNameQualifiedMutation::Vanilla(vanilla) => {
                self.remove_vanilla_specialty(*vanilla, &remove_specialty.specialty)
            }
            AbilityNameQualifiedMutation::Craft(focus) => {
                self.remove_craft_specialty(focus, &remove_specialty.specialty)
            }
            AbilityNameQualifiedMutation::MartialArts(style) => {
                self.remove_martial_arts_specialty(style, &remove_specialty.specialty)
            }
        }
    }

    /// Removes a specialty from a non-Craft, non-Martial Arts ability.
    pub fn remove_vanilla_specialty(
        &mut self,
        ability_name: AbilityNameVanilla,
        specialty: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.abilities
            .get_mut(ability_name)
            .remove_specialty(specialty)?;
        Ok(self)
    }
}
