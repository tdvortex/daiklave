use std::{ops::Deref, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::{id::UniqueId, CharacterMutationError};

mod character;
mod character_view;
mod error;
mod essence_view;
mod exalt_state_view;
mod exalt_state;
mod exalt;
mod exalt_view;

pub use error::{
    CommitMotesError, RecoverMotesError, SetEssenceRatingError, SpendMotesError, UncommitMotesError,
};

pub(crate) use essence_view::{EssenceView};

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

    pub fn motes_mut(&mut self) -> &mut Motes {
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

    pub fn peripheral_mut(&mut self) -> &mut MoteState {
        &mut self.peripheral
    }

    pub fn personal(&self) -> &MoteState {
        &self.personal
    }

    pub fn personal_mut(&mut self) -> &mut MoteState {
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



/// A unique identifier for a mote commitment effect.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommittedMotesId(pub UniqueId);

impl Deref for CommittedMotesId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

