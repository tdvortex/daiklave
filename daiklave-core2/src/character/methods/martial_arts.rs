use crate::{
    martial_arts::{
        charm::{MartialArtsCharm, MartialArtsCharmId},
        MartialArts, MartialArtsStyle, MartialArtsStyleId,
    },
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
        id: MartialArtsStyleId,
        style: &'source MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_martial_arts_style(id, style)?;

        Ok(self)
    }

    /// Removes a Martial Arts style from the character.
    pub fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.remove_martial_arts_style(id)?;
        Ok(self)
    }

    /// Sets the ability dots for a specific Martial Arts style.
    pub fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.set_martial_arts_dots(id, dots)?;
        Ok(self)
    }

    /// Adds a Martial Arts Charm to the character.
    pub fn add_martial_arts_charm(
        &mut self,
        martial_arts_charm_id: MartialArtsCharmId,
        martial_arts_charm: &'source MartialArtsCharm,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.add_martial_arts_charm(martial_arts_charm_id, martial_arts_charm)?;
        Ok(self)
    }

    /// Removes a Martial Arts Charm from the character.
    pub fn remove_martial_arts_charm(
        &mut self,
        martial_arts_charm_id: MartialArtsCharmId,
    ) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }
}
