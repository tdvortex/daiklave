/// Structs and methods related to the Essence rating and mote pools for a
/// character.
pub mod essence;

/// Structs and methods related to various Exalt subtypes (Solar, Lunar, etc).
pub mod exalt_type;
use std::collections::HashMap;

use essence::{Essence, EssenceView};
use exalt_type::{ExaltType, ExaltTypeView};
use serde::{Deserialize, Serialize};

use crate::{martial_arts::{ExaltMartialArtist, ExaltMartialArtistView, MartialArtsStyleId}, CharacterMutationError};

use self::{essence::{MoteCommitmentId, SetEssenceRatingError, MotePoolName, SpendMotesError, CommitMotesError, MoteCommitmentView, UncommitMotesError, MoteCommitment}, exalt_type::solar::Solar};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Exalt {
    essence: Essence,
    pub martial_arts_styles: HashMap<MartialArtsStyleId, ExaltMartialArtist>,
    pub exalt_type: ExaltType,
}

impl Exalt {
    pub fn is_solar(&self) -> bool {
        self.exalt_type.is_solar()
    }

    pub fn solar_traits(&self) -> Option<&Solar> {
        self.exalt_type.solar_traits()
    }

    pub fn essence(&self) -> &Essence {
        &self.essence
    }

    pub fn essence_mut(&mut self) -> &mut Essence {
        &mut self.essence
    }

    pub fn check_spend_motes(
        &self,
        _first: MotePoolName,
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

    pub fn spend_motes(
        &mut self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_spend_motes(first, amount)?;

        let (peripheral_spent, personal_spent) = if let MotePoolName::Peripheral = first {
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

    pub fn check_commit_motes(
        &self,
        _id: &MoteCommitmentId,
        _name: &str,
        _first: MotePoolName,
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

    pub fn commit_motes(
        &mut self,
        id: &MoteCommitmentId,
        name: &str,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_commit_motes(id, name, first, amount)?;
        let (peripheral_committed, personal_committed) = if let MotePoolName::Peripheral = first {
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

    pub fn recover_motes(
        &mut self,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
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

    pub fn check_uncommit_motes(
        &self,
        id: &MoteCommitmentId,
    ) -> Result<(), CharacterMutationError> {
        if !self.essence().motes().commitments.contains_key(id) {
            Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::NotFound(*id),
            ))
        } else {
            Ok(())
        }
    }

    pub fn uncommit_motes(
        &mut self,
        id: &MoteCommitmentId,
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

    pub fn set_essence_rating(
        &mut self,
        rating: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.essence().rating() == rating {
            return Ok(self);
        }

        if !(1..=5).contains(&rating) {
            return Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::InvalidRating(rating),
            ));
        }

        let (new_peripheral, new_personal) = match self.exalt_type {
            ExaltType::Solar(_) => (rating * 7 + 26, rating * 3 + 10),
        };

        let committed_ids = self
            .essence()
            .motes()
            .committed()
            .map(|x| x.0)
            .collect::<Vec<MoteCommitmentId>>();
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


#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExaltView<'source> {
    essence: EssenceView<'source>,
    pub martial_arts_styles: HashMap<MartialArtsStyleId, ExaltMartialArtistView<'source>>,
    pub exalt_type: ExaltTypeView<'source>,
}

impl<'source> ExaltView<'source> {
    pub fn essence(&self) -> &EssenceView {
        &self.essence
    }

    pub fn essence_mut(&mut self) -> &mut EssenceView<'source> {
        &mut self.essence
    }

    pub fn check_spend_motes(
        &self,
        _first: MotePoolName,
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

    pub fn spend_motes(
        &mut self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_spend_motes(first, amount)?;

        let (peripheral_spent, personal_spent) = if let MotePoolName::Peripheral = first {
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

    pub fn check_commit_motes(
        &self,
        _id: &MoteCommitmentId,
        _name: &str,
        _first: MotePoolName,
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

    pub fn commit_motes(
        &mut self,
        id: &MoteCommitmentId,
        name: &'source str,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_commit_motes(id, name, first, amount)?;
        let (peripheral_committed, personal_committed) = if let MotePoolName::Peripheral = first {
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
        let commitment = MoteCommitmentView {
            name,
            peripheral: peripheral_committed,
            personal: personal_committed,
        };
        self.essence_mut()
            .motes_mut()
            .commitments
            .insert(*id, commitment);
        Ok(self)
    }

    pub fn recover_motes(
        &mut self,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
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

    pub fn check_uncommit_motes(
        &self,
        id: &MoteCommitmentId,
    ) -> Result<(), CharacterMutationError> {
        if !self.essence().motes().commitments.contains_key(id) {
            Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::NotFound(*id),
            ))
        } else {
            Ok(())
        }
    }

    pub fn uncommit_motes(
        &mut self,
        id: &MoteCommitmentId,
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

    pub fn set_essence_rating(
        &mut self,
        rating: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.essence().rating() == rating {
            return Ok(self);
        }

        if !(1..=5).contains(&rating) {
            return Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::InvalidRating(rating),
            ));
        }

        let (new_peripheral, new_personal) = match self.exalt_type {
            ExaltTypeView::Solar(_) => (rating * 7 + 26, rating * 3 + 10),
        };

        let committed_ids = self
            .essence()
            .motes()
            .committed()
            .map(|x| x.0)
            .collect::<Vec<MoteCommitmentId>>();
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
