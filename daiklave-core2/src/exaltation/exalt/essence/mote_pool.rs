use serde::{Serialize, Deserialize};

use crate::CharacterMutationError;

use super::{SpendMotesError, CommitMotesError};

/// The available and spent motes from either a peripheral or personal pool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct MotePool {
    pub(crate) available: u8,
    pub(crate) spent: u8,
}

impl MotePool {
    /// The available motes from the specific pool.
    pub fn available(&self) -> u8 {
        self.available
    }

    /// The spent (but not committed) motes from the specific pool.
    pub fn spent(&self) -> u8 {
        self.spent
    }

    /// Spend a number of motes, shifting them from available to spent.
    pub fn spend(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
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

    /// Commit a number of motes, removing the from the pool and locking them
    /// inside the committed effect.
    pub fn commit(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        if amount > self.available {
            Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::InsufficientMotes(self.available, amount),
            ))
        } else {
            self.available -= amount;
            Ok(self)
        }
    }

    /// Recover spent motes, shifting them from spent to available.
    pub fn recover(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        let recovered = amount.min(self.spent);
        self.spent -= recovered;
        self.available += recovered;
        Ok(self)
    }

    /// Recover motes from a commitment effect, adding these to spent, where
    /// they can later be recovered.
    pub fn uncommit(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        self.spent += amount;
        Ok(self)
    }
}