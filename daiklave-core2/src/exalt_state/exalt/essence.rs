use std::{collections::HashMap, ops::Deref};

use serde::{Deserialize, Serialize};

use crate::{id::UniqueId, CharacterMutationError};

mod character;
mod character_view;
mod error;
mod essence_view;
mod exalt;
mod exalt_state;
mod exalt_state_view;
mod exalt_view;

pub use error::{
    CommitMotesError, RecoverMotesError, SetEssenceRatingError, SpendMotesError, UncommitMotesError,
};

pub(crate) use essence_view::EssenceView;

/// The current state of a character's Essence and motes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Essence {
    pub(crate) rating: u8,
    pub(crate) motes: Motes,
}

impl Essence {
    /// The chacter's Essence dot rating.
    pub fn rating(&self) -> u8 {
        self.rating
    }

    /// The character's current mote pool state.
    pub fn motes(&self) -> &Motes {
        &self.motes
    }

    pub(crate) fn motes_mut(&mut self) -> &mut Motes {
        &mut self.motes
    }
}

/// The current state of a character's mote balances. Includes both peripheral
/// and personal mote pools, as well as commitments from both.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Motes {
    pub(crate) peripheral: MoteState,
    pub(crate) personal: MoteState,
    pub(crate) commitments: HashMap<CommittedMotesId, MoteCommitment>,
}

impl Motes {
    /// The character's current available and spent peripheral motes.
    pub fn peripheral(&self) -> &MoteState {
        &self.peripheral
    }

    pub(crate) fn peripheral_mut(&mut self) -> &mut MoteState {
        &mut self.peripheral
    }

    /// The character's current available and spent personal motes.
    pub fn personal(&self) -> &MoteState {
        &self.personal
    }

    pub(crate) fn personal_mut(&mut self) -> &mut MoteState {
        &mut self.personal
    }

    /// An iterator over the character's current mote commitments, structured
    /// as (id, name, peripheral committed, personal committed).
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

/// The available and spent motes from either a peripheral or personal pool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct MoteState {
    pub(crate) available: u8,
    pub(crate) spent: u8,
}

impl MoteState {
    /// The available motes from the specific pool.
    pub fn available(&self) -> u8 {
        self.available
    }

    /// The spent (but not committed) motes from the specific pool.
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
