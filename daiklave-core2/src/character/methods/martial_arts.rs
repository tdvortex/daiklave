use crate::{
    martial_arts::{charm::AddMartialArtsCharm, style::MartialArtsStyle, MartialArts},
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    /// Accesses Martial Arts styles, abilities, and Charms.
    pub fn martial_arts(&'view self) -> MartialArts<'view, 'source> {
        MartialArts(&self.exaltation)
    }

    /// Adds a Martial Arts style to the character.
    pub fn add_martial_arts_style(
        &mut self,
        name: &'source str,
        style: &'source MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_martial_arts_style(name, style)?;

        Ok(self)
    }

    /// Removes a Martial Arts style from the character.
    pub fn remove_martial_arts_style(
        &mut self,
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.remove_martial_arts_style(name)?;
        Ok(self)
    }

    /// Sets the ability dots for a specific Martial Arts style.
    pub fn set_martial_arts_dots(
        &mut self,
        name: &str,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.set_martial_arts_dots(name, dots)?;
        Ok(self)
    }

    /// Adds a specialty to a Martial Arts style ability.
    pub fn add_martial_arts_specialty(
        &mut self,
        style_name: &'source str,
        specialty: &'source str,
    ) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }

    /// Removes a specialty from a Martial Arts style ability.
    pub fn remove_martial_arts_specialty(
        &mut self,
        style_name: &str,
        specialty: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }

    /// Adds a Martial Arts Charm to the character.
    pub fn add_martial_arts_charm(
        &mut self,
        add_martial_arts_charm: &'source AddMartialArtsCharm,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_martial_arts_charm(
            &add_martial_arts_charm.style,
            &add_martial_arts_charm.name,
            &add_martial_arts_charm.charm,
        )?;
        Ok(self)
    }

    pub(crate) fn correct_martial_arts_charms(&mut self, force_remove: &[&str]) -> bool {
        self.exaltation.correct_martial_arts_charms(force_remove)
    }

    /// Removes a Martial Arts Charm from the character.
    pub fn remove_martial_arts_charm(
        &mut self,
        martial_arts_charm_name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation
            .remove_martial_arts_charm(martial_arts_charm_name)?;

        // Evocations may be upgrades to Martial Arts Charms
        // Removing a Martial Arts charm may force removal of an Evocation
        self.correct_evocations(&[]);
        Ok(self)
    }
}
