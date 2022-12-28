use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    exalt_type::{ExaltState, ExaltType},
    Character, CharacterMutationError, CommittedMotesId, MotePool,
};

use super::{
    CommitMotesError, MoteState, RecoverMotesError, SetEssenceRatingError, SpendMotesError,
    UncommitMotesError,
};

impl Character {
    /// None for mortals.
    pub fn essence(&self) -> Option<&Essence> {
        self.exalt_state.essence()
    }

    /// Checks if the requested amount of motes can be spent.
    pub fn check_spend_motes(
        &self,
        first: MotePool,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_spend_motes(first, amount)
    }

    /// Spends motes, starting with the specified pool first.
    pub fn spend_motes(
        &mut self,
        first: MotePool,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.spend_motes(first, amount)?;
        Ok(self)
    }

    /// Checks if the requested mote commitment would be possible.
    pub fn check_commit_motes(
        &self,
        id: &CommittedMotesId,
        name: &str,
        first: MotePool,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_commit_motes(id, name, first, amount)
    }

    /// Removes available motes, starting with the specified pool, and
    /// packages them into a commitment package to be later uncommitted.
    pub fn commit_motes(
        &mut self,
        id: &CommittedMotesId,
        name: &str,
        first: MotePool,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.commit_motes(id, name, first, amount)?;
        Ok(self)
    }

    /// Checks if mote recovery is possible.
    pub fn check_recover_motes(&self, amount: u8) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_recover_motes(amount)
    }

    /// Recovers motes, moving them from spent to available. Will not uncommit
    /// motes.
    pub fn recover_motes(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.recover_motes(amount)?;
        Ok(self)
    }

    /// Checks if a committed mote effect can be uncommitted.
    pub fn check_uncommit_motes(
        &self,
        id: &CommittedMotesId,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_uncommit_motes(id)
    }

    /// Uncommits a mote effect, returning the committed motes to their pool(s)
    /// as spent motes to be later recovered.
    pub fn uncommit_motes(
        &mut self,
        id: &CommittedMotesId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.uncommit_motes(id)?;
        Ok(self)
    }

    /// Checks if essence can be set to the specified value.
    pub fn check_set_essence_rating(&self, rating: u8) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_essence_rating(rating)
    }

    /// Changes the essence rating of the character to the specified value.
    /// This also uncommits all active effects and recovers all motes.
    pub fn set_essence_rating(&mut self, rating: u8) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.set_essence_rating(rating)?;
        Ok(self)
    }
}

impl ExaltState {
    fn essence(&self) -> Option<&Essence> {
        match self {
            ExaltState::Mortal => None,
            ExaltState::Exalted(exalt_type) => Some(exalt_type.essence()),
        }
    }

    fn check_spend_motes(&self, first: MotePool, amount: u8) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            ExaltState::Exalted(exalt_type) => exalt_type.check_spend_motes(first, amount),
        }
    }

    fn spend_motes(
        &mut self,
        first: MotePool,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            ExaltState::Exalted(exalt_type) => exalt_type.spend_motes(first, amount),
        }?;
        Ok(self)
    }

    fn check_commit_motes(
        &self,
        id: &CommittedMotesId,
        name: &str,
        first: MotePool,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            ExaltState::Exalted(exalt_type) => {
                exalt_type.check_commit_motes(id, name, first, amount)
            }
        }
    }

    fn commit_motes(
        &mut self,
        id: &CommittedMotesId,
        name: &str,
        first: MotePool,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            ExaltState::Exalted(exalt_type) => exalt_type.commit_motes(id, name, first, amount),
        }?;
        Ok(self)
    }

    fn check_recover_motes(&self, _amount: u8) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            ExaltState::Exalted(_) => Ok(()),
        }
    }

    fn recover_motes(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            ExaltState::Exalted(exalt_type) => exalt_type.recover_motes(amount),
        }?;
        Ok(self)
    }

    fn check_uncommit_motes(&self, id: &CommittedMotesId) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            ExaltState::Exalted(exalt_type) => exalt_type.check_uncommit_motes(id),
        }
    }

    fn uncommit_motes(
        &mut self,
        id: &CommittedMotesId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            ExaltState::Exalted(exalt_type) => exalt_type.uncommit_motes(id),
        }?;
        Ok(self)
    }

    fn check_set_essence_rating(&self, rating: u8) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
            ExaltState::Exalted(_) => {
                if (1..=5).contains(&rating) {
                    Ok(())
                } else {
                    Err(CharacterMutationError::SetEssenceRatingError(
                        SetEssenceRatingError::InvalidRating(rating),
                    ))
                }
            }
        }
    }

    fn set_essence_rating(&mut self, rating: u8) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_essence_rating(rating)?;
        match self {
            ExaltState::Exalted(exalt_type) => exalt_type.set_essence_rating(rating),
            ExaltState::Mortal => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
        }?;
        Ok(self)
    }
}

impl ExaltType {
    fn essence(&self) -> &Essence {
        match self {
            ExaltType::Solar(solar_traits) => &solar_traits.essence,
        }
    }

    fn essence_mut(&mut self) -> &mut Essence {
        match self {
            ExaltType::Solar(solar_traits) => &mut solar_traits.essence,
        }
    }

    fn check_spend_motes(
        &self,
        _first: MotePool,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        let total_available = self.essence().motes().peripheral().available()
            + self.essence().motes().personal().available();

        if total_available < amount {
            Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::InsufficientMotes(total_available, amount),
            ))
        } else {
            Ok(())
        }
    }

    fn spend_motes(
        &mut self,
        first: MotePool,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_spend_motes(first, amount)?;

        let (peripheral_spent, personal_spent) = if let MotePool::Peripheral = first {
            let peripheral_spent = self.essence().motes().peripheral().available().min(amount);
            let personal_spent = amount - peripheral_spent;
            (peripheral_spent, personal_spent)
        } else {
            let personal_spent = self.essence().motes().personal().available().min(amount);
            let peripheral_spent = amount - personal_spent;
            (peripheral_spent, personal_spent)
        };

        self.essence_mut()
            .motes_mut()
            .peripheral_mut()
            .spend(peripheral_spent)?;
        self.essence_mut()
            .motes_mut()
            .personal_mut()
            .spend(personal_spent)?;
        Ok(self)
    }

    fn check_commit_motes(
        &self,
        _id: &CommittedMotesId,
        _name: &str,
        _first: MotePool,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        let total_available = self.essence().motes().peripheral().available()
            + self.essence().motes().personal().available();

        if total_available < amount {
            Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::InsufficientMotes(total_available, amount),
            ))
        } else {
            Ok(())
        }
    }

    fn commit_motes(
        &mut self,
        id: &CommittedMotesId,
        name: &str,
        first: MotePool,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_commit_motes(id, name, first, amount)?;
        let (peripheral_committed, personal_committed) = if let MotePool::Peripheral = first {
            let peripheral_committed = self.essence().motes().peripheral().available().min(amount);
            let personal_committed = amount - peripheral_committed;
            (peripheral_committed, personal_committed)
        } else {
            let personal_committed = self.essence().motes().personal().available().min(amount);
            let peripheral_committed = amount - personal_committed;
            (peripheral_committed, personal_committed)
        };

        self.essence_mut()
            .motes_mut()
            .peripheral_mut()
            .commit(peripheral_committed)?;
        self.essence_mut()
            .motes_mut()
            .personal_mut()
            .commit(personal_committed)?;
        let commitment = MoteCommitment {
            name: name.to_string(),
            peripheral: peripheral_committed,
            personal: personal_committed,
        };
        self.essence_mut()
            .motes_mut()
            .commitments
            .insert(*id, commitment);
        Ok(self)
    }

    fn recover_motes(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        let peripheral_recovered = self.essence().motes().peripheral().spent().min(amount);
        let personal_recovered = self
            .essence()
            .motes()
            .personal()
            .spent()
            .min(amount - peripheral_recovered);

        self.essence_mut()
            .motes_mut()
            .peripheral_mut()
            .recover(peripheral_recovered)?;
        self.essence_mut()
            .motes_mut()
            .personal_mut()
            .recover(personal_recovered)?;
        Ok(self)
    }

    fn check_uncommit_motes(&self, id: &CommittedMotesId) -> Result<(), CharacterMutationError> {
        if !self.essence().motes().commitments.contains_key(id) {
            Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::NotFound(*id),
            ))
        } else {
            Ok(())
        }
    }

    fn uncommit_motes(
        &mut self,
        id: &CommittedMotesId,
    ) -> Result<&mut Self, CharacterMutationError> {
        let commitment = self
            .essence_mut()
            .motes_mut()
            .commitments
            .remove(id)
            .ok_or({
                CharacterMutationError::UncommitMotesError(UncommitMotesError::NotFound(*id))
            })?;
        self.essence_mut()
            .motes_mut()
            .peripheral_mut()
            .uncommit(commitment.peripheral)
            .unwrap();
        self.essence_mut()
            .motes_mut()
            .personal_mut()
            .uncommit(commitment.personal)
            .unwrap();
        Ok(self)
    }

    fn set_essence_rating(&mut self, rating: u8) -> Result<&mut Self, CharacterMutationError> {
        if self.essence().rating() == rating {
            return Ok(self);
        }

        if !(1..=5).contains(&rating) {
            return Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::InvalidRating(rating),
            ));
        }

        let (new_peripheral, new_personal) = match self {
            ExaltType::Solar(_) => (rating * 7 + 26, rating * 3 + 10),
        };

        let committed_ids = self
            .essence()
            .motes()
            .committed()
            .map(|x| x.0)
            .collect::<Vec<CommittedMotesId>>();
        for id in committed_ids {
            self.uncommit_motes(&id).unwrap();
        }

        let spent_peripheral = self.essence().motes().peripheral().spent();
        self.essence_mut()
            .motes_mut()
            .peripheral_mut()
            .recover(spent_peripheral)
            .unwrap();
        let available_peripheral = self.essence().motes().peripheral().available();
        if available_peripheral < new_peripheral {
            self.essence_mut()
                .motes_mut()
                .peripheral_mut()
                .uncommit(new_peripheral - available_peripheral)
                .unwrap()
                .recover(new_peripheral - available_peripheral)
                .unwrap();
        } else {
            self.essence_mut()
                .motes_mut()
                .peripheral_mut()
                .commit(available_peripheral - new_peripheral)
                .unwrap();
        }

        let spent_personal = self.essence().motes().personal().spent();
        self.essence_mut()
            .motes_mut()
            .personal_mut()
            .recover(spent_personal)
            .unwrap();
        let available_personal = self.essence().motes().personal().available();
        if available_personal < new_personal {
            self.essence_mut()
                .motes_mut()
                .personal_mut()
                .uncommit(new_personal - available_personal)
                .unwrap()
                .recover(new_personal - available_personal)
                .unwrap();
        } else {
            self.essence_mut()
                .motes_mut()
                .peripheral_mut()
                .commit(available_personal - new_personal)
                .unwrap();
        }

        self.essence_mut().rating = rating;

        Ok(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Essence {
    pub(crate) rating: u8,
    pub(crate) motes: Motes,
}

impl Essence {
    pub fn rating(&self) -> u8 {
        self.rating
    }

    pub fn motes(&self) -> &Motes {
        &self.motes
    }

    fn motes_mut(&mut self) -> &mut Motes {
        &mut self.motes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Motes {
    pub(crate) peripheral: MoteState,
    pub(crate) personal: MoteState,
    pub(crate) commitments: HashMap<CommittedMotesId, MoteCommitment>,
}

impl Motes {
    pub fn peripheral(&self) -> &MoteState {
        &self.peripheral
    }

    fn peripheral_mut(&mut self) -> &mut MoteState {
        &mut self.peripheral
    }

    pub fn personal(&self) -> &MoteState {
        &self.personal
    }

    fn personal_mut(&mut self) -> &mut MoteState {
        &mut self.personal
    }

    pub fn committed(&self) -> impl Iterator<Item = (CommittedMotesId, &str, u8, u8)> {
        self.commitments
            .iter()
            .map(|(k, v)| (*k, v.name.as_str(), v.peripheral, v.personal))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub(crate) struct MoteCommitment {
    name: String,
    peripheral: u8,
    personal: u8,
}
