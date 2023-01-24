use std::num::NonZeroU16;

use serde::{Deserialize, Serialize};

use crate::CharacterMutationError;

use super::ExperienceError;

/// One pool of Experience points, whether normal or tied to a specific Exalt
/// type (like Solar Experience)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct ExperiencePool {
    pub(crate) unspent: u16,
    pub(crate) spent: u16,
}

impl ExperiencePool {
    /// The current unspent experience points.
    pub fn current(&self) -> u16 {
        self.unspent
    }

    /// The total number of experience points, both 
    pub fn total(&self) -> u16 {
        self.unspent.saturating_add(self.spent)
    }

    pub(crate) fn gain(&mut self, amount: NonZeroU16) {
        self.unspent = self.unspent.saturating_add(amount.get());
    }

    pub(crate) fn spend(
        &mut self,
        amount: NonZeroU16,
    ) -> Result<&mut Self, CharacterMutationError> {
        if amount.get() > self.unspent {
            Err(CharacterMutationError::ExperienceError(
                ExperienceError::InsufficientExperience,
            ))
        } else {
            self.unspent -= amount.get();
            self.spent = self.spent.saturating_add(amount.get());
            Ok(self)
        }
    }
}
