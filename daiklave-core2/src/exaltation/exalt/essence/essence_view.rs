use std::collections::HashMap;

use super::{mote_pool::MotePool, motes_view::MotesView};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EssenceView<'source> {
    pub(crate) rating: u8,
    pub(crate) motes: MotesView<'source>,
}

impl<'source> EssenceView<'source> {
    pub fn rating(&self) -> u8 {
        self.rating
    }

    pub fn motes(&self) -> &MotesView {
        &self.motes
    }

    pub fn motes_mut(&mut self) -> &mut MotesView<'source> {
        &mut self.motes
    }

    pub(crate) fn new_solar(rating: u8) -> Self {
        Self {
            rating,
            motes: MotesView {
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
