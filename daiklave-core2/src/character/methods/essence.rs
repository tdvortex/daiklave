use crate::{
    exaltation::exalt::essence::{Essence, MoteCommitmentId, MotePoolName, OtherMoteCommitmentId},
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    /// None for mortals.
    pub fn essence(&'view self) -> Option<Essence<'view, 'source>> {
        self.exaltation.essence()
    }

    /// Checks if the requested amount of motes can be spent.
    pub fn check_spend_motes(
        &self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        self.exaltation.check_spend_motes(first, amount)
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

    /// Checks if the requested mote commitment would be possible.
    pub fn check_commit_motes(
        &self,
        id: &OtherMoteCommitmentId,
        name: &str,
        first: MotePoolName,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        self.exaltation.check_commit_motes(id, name, first, amount)
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

    /// Checks if mote recovery is possible.
    pub fn check_recover_motes(&self, amount: u8) -> Result<(), CharacterMutationError> {
        self.exaltation.check_recover_motes(amount)
    }

    /// Recovers motes, moving them from spent to available. Will not uncommit
    /// motes.
    pub fn recover_motes(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.recover_motes(amount)?;
        Ok(self)
    }

    /// Checks if a committed mote effect can be uncommitted.
    pub fn check_uncommit_motes(
        &self,
        id: &MoteCommitmentId,
    ) -> Result<(), CharacterMutationError> {
        self.exaltation.check_uncommit_motes(id)
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

    /// Checks if essence can be set to the specified value.
    pub fn check_set_essence_rating(&self, rating: u8) -> Result<(), CharacterMutationError> {
        self.exaltation.check_set_essence_rating(rating)
    }

    /// Changes the essence rating of the character to the specified value.
    /// This also uncommits all active effects and recovers all motes.
    pub fn set_essence_rating(&mut self, rating: u8) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.set_essence_rating(rating)?;
        Ok(self)
    }
}
