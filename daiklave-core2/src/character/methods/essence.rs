use crate::{
    exaltation::exalt::essence::{Essence, MoteCommitmentId, MotePoolName, OtherMoteCommitmentId},
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
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.spend_motes(first, amount)?;
        Ok(self)
    }

    /// Removes available motes, starting with the specified pool, and
    /// packages them into a commitment package to be later uncommitted.
    pub fn commit_motes(
        &mut self,
        id: &OtherMoteCommitmentId,
        name: &'source str,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.commit_motes(id, name, first, amount)?;
        Ok(self)
    }

    /// Recovers motes, moving them from spent to available. Will not uncommit
    /// motes.
    pub fn recover_motes(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.recover_motes(amount)?;
        Ok(self)
    }

    /// Uncommits a mote effect, returning the committed motes to their pool(s)
    /// as spent motes to be later recovered.
    pub fn uncommit_motes(
        &mut self,
        id: &MoteCommitmentId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.uncommit_motes(id)?;
        Ok(self)
    }

    /// Changes the essence rating of the character to the specified value.
    /// This also uncommits all active effects and recovers all motes. If
    /// the rating is decreased, may cause Charms or Sorcery to be lost.
    pub fn set_essence_rating(&mut self, rating: u8) -> Result<&mut Self, CharacterMutationError> {
        let old_rating = self.essence().map(|essence| essence.rating()).unwrap_or(0);
        if old_rating == rating {
            return Ok(self)
        }        
        self.exaltation.set_essence_rating(rating)?;
        if old_rating > rating {
            self.correct_solar_charms(&[]);
        }
        
        Ok(self)
    }
}
