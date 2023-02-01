use crate::{
    abilities::AbilityNameVanilla,
    attributes::AttributeName,
    sorcery::{
        circles::{
            celestial::AddCelestialSorcery,
            solar::AddSolarSorcery,
            terrestrial::{AddTerrestrialSorcery},
        },
        spell::SpellMutation,
        Sorcery, AddSorcery, AddSorceryCircle,
    },
    Character, CharacterMutationError, merits::merit_new::AddSorceryArchetypeMerit,
};

impl<'view, 'source> Character<'source> {
    /// The character's Sorcery abilities, if any.
    pub fn sorcery(&'view self) -> Option<Sorcery<'view, 'source>> {
        self.exaltation.sorcery()
    }

    /// If the character was not already a sorcerer, adds the first circle of
    /// sorcery.
    pub fn add_terrestrial_sorcery(
        &mut self,
        add_terrestrial: &'source AddTerrestrialSorcery,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_terrestrial_sorcery(
            add_terrestrial,
            self.abilities().get_vanilla(AbilityNameVanilla::Occult).dots(),
            self.attributes().get(AttributeName::Intelligence).dots(),
        )?;
        Ok(self)
    }

    /// Removes Terrestrial circle sorcery from the character.
    pub fn remove_terrestrial_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.remove_terrestrial_sorcery()?;
        Ok(self)
    }

    /// Upgrades the character from Terrestrial to Celestial sorcery.
    pub fn add_celestial_sorcery(
        &mut self,
        add_celestial: &'source AddCelestialSorcery,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_celestial_sorcery(
            add_celestial,
            self.abilities().get_vanilla(AbilityNameVanilla::Occult).dots(),
            self.attributes().get(AttributeName::Intelligence).dots(),
            self.essence().map_or(1, |essence| essence.rating()),
        )?;
        Ok(self)
    }

    /// Removes Celestial circle sorcery from the character.
    pub fn remove_celestial_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.remove_celestial_sorcery()?;
        Ok(self)
    }

    /// Upgrades the character from Celestial to Solar sorcery.
    pub fn add_solar_sorcery(
        &mut self,
        add_solar: &'source AddSolarSorcery,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_solar_sorcery(
            add_solar,
            self.abilities().get_vanilla(AbilityNameVanilla::Occult).dots(),
            self.essence().map_or(1, |essence| essence.rating()),
        )?;
        Ok(self)
    }

    /// Removes Solar circle sorcery from the character.
    pub fn remove_solar_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.remove_solar_sorcery()?;
        Ok(self)
    }

    /// Removes the highest level of sorcery the character has attained.
    pub fn remove_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.remove_solar_sorcery().is_ok() {
            Ok(self)
        } else if self.remove_celestial_sorcery().is_ok() {
            Ok(self)
        } else {
            self.remove_terrestrial_sorcery()
        }
    }

    /// Adds a circle of sorcery to the character.
    pub fn add_sorcery(&mut self, add_sorcery: &'source AddSorcery) -> Result<&mut Self, CharacterMutationError> {
        match &add_sorcery.0.as_ref() {
            AddSorceryCircle::Terrestrial(add_terrestrial) => self.add_terrestrial_sorcery(add_terrestrial),
            AddSorceryCircle::Celestial(add_celestial) => self.add_celestial_sorcery(add_celestial),
            AddSorceryCircle::Solar(add_solar) => self.add_solar_sorcery(add_solar),
        }
    }

    /// Adds a merit to a Sorcery Archetype owned by the character
    pub fn add_sorcery_archetype_merit(
        &mut self,
        add_sorcery_archetype_merit: &'source AddSorceryArchetypeMerit,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_sorcery_archetype_merit(
            &add_sorcery_archetype_merit.archetype_name,
            &add_sorcery_archetype_merit.name,
            &add_sorcery_archetype_merit.details,
        )?;
        Ok(self)
    }

    /// Removes a merit from a Sorcery Archetype owned by a character
    pub fn remove_sorcery_archetype_merit(
        &mut self,
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation
            .remove_sorcery_archetype_merit(name)?;
        Ok(self)
    }

    /// Adds a Spell to the character.
    pub fn add_spell(
        &mut self,
        name: &'source str,
        spell: &'source SpellMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_spell(name, spell)?;
        Ok(self)
    }

    /// Removes a Spell from the character. Control Spells cannot be removed.
    pub fn remove_spell(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.remove_spell(name)?;
        self.correct_evocations(&[]);
        Ok(self)
    }

    pub(crate) fn correct_sorcery_level(&mut self) -> bool {
        let occult_dots = self.abilities().get_vanilla(AbilityNameVanilla::Occult).dots();
        let intelligence_dots = self.attributes().get(AttributeName::Intelligence).dots();
        let essence_rating = self.essence().map(|essence| essence.rating()).unwrap_or(0);
        self.exaltation
            .correct_sorcery_level(occult_dots, intelligence_dots, essence_rating)
    }
}
