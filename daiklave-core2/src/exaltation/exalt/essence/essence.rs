use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use super::{motes::Motes, mote_pool::MotePool};

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

    pub(crate) fn new_solar(rating: u8) -> Self {
        Self {
            rating,
            motes: Motes {
                peripheral: MotePool {
                    available: rating * 7 + 26,
                    spent: 0,
                },
                personal: MotePool {
                    available: rating * 3 + 10,
                    spent: 0,
                },
                commitments: HashMap::new(),
            },
        }
    }
}