use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::{id::Id, CharacterMutationError};

mod character;
mod character_view;
mod error;
pub use character::{Essence, Motes};
pub use character_view::{EssenceView, MotesView};
pub use error::{
    CommitMotesError, RecoverMotesError, SetEssenceRatingError, SpendMotesError, UncommitMotesError,
};

/// Indicates whether motes are spent/committed from peripheral or peripheral
/// pool first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum MotePool {
    /// Spend/commit peripheral motes first
    Peripheral,
    /// Spend/commit personal motes first
    Personal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct MoteState {
    pub(crate) available: u8,
    pub(crate) spent: u8,
}

impl MoteState {
    pub fn available(&self) -> u8 {
        self.available
    }

    pub fn spent(&self) -> u8 {
        self.spent
    }

    fn spend(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        if amount > self.available {
            Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::InsufficientMotes(amount, self.available),
            ))
        } else {
            self.available -= amount;
            self.spent += amount;
            Ok(self)
        }
    }

    fn commit(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        if amount > self.available {
            Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::InsufficientMotes(self.available, amount),
            ))
        } else {
            self.available -= amount;
            Ok(self)
        }
    }

    fn recover(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        let recovered = amount.min(self.spent);
        self.spent -= recovered;
        self.available += recovered;
        Ok(self)
    }

    fn uncommit(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        self.spent += amount;
        Ok(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MoteCommitmentView<'source> {
    pub(crate) name: &'source str,
    pub(crate) peripheral: u8,
    pub(crate) personal: u8,
}

/// A unique identifier for a mote commitment effect.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommittedMotesId(pub Id);

impl Deref for CommittedMotesId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
