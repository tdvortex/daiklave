use std::num::NonZeroU8;

use crate::{
    exaltation::exalt::essence::{Essence, MotePoolName, UncommitMotes},
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    /// None for mortals.
    pub fn essence(&'view self) -> Option<Essence<'view, 'source>> {
        self.exaltation.essence()
    }

    /// Spends motes, starting with the specified pool first.
    pub fn spend_motes(
        &mut self,
        first: MotePoolName,
        amount: NonZeroU8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.spend_motes(first, amount)?;
        Ok(self)
    }

    /// Removes available motes, starting with the specified pool, and
    /// packages them into a commitment package to be later uncommitted.
    pub fn commit_motes(
        &mut self,
        name: &'source str,
        first: MotePoolName,
        amount: NonZeroU8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.commit_motes(name, first, amount)?;
        Ok(self)
    }

    /// Recovers motes, moving them from spent to available. Will not uncommit
    /// motes.
    pub fn recover_motes(&mut self, amount: NonZeroU8) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.recover_motes(amount)?;
        Ok(self)
    }

    /// Uncommits a mote effect, returning the committed motes to their pool(s)
    /// as spent motes to be later recovered.
    pub fn uncommit_motes(
        &mut self,
        name: &'source UncommitMotes,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.uncommit_motes(name)?;
        Ok(self)
    }

    /// Changes the essence rating of the character to the specified value.
    /// This also uncommits all active effects and recovers all motes. If
    /// the rating is decreased, may cause Charms or Sorcery to be lost.
    pub fn set_essence_rating(&mut self, rating: NonZeroU8) -> Result<&mut Self, CharacterMutationError> {
        let old_rating = self.essence().map(|essence| essence.rating()).unwrap_or(0);
        if old_rating == rating.get() {
            return Ok(self);
        }
        self.exaltation.set_essence_rating(rating)?;
        if old_rating > rating.get() {
            self.correct_sorcery_level();
            self.correct_solar_charms(&[]);
            self.correct_eclipse_charms(&[]);
            self.correct_martial_arts_charms(&[]);
            self.correct_evocations(&[]);
        }

        Ok(self)
    }
}
