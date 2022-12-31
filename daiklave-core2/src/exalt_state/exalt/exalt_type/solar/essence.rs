use std::collections::HashMap;

use crate::exalt_state::exalt::essence::{Essence, EssenceView, MoteState, Motes, MotesView};

impl Essence {
    pub(crate) fn new_solar(rating: u8) -> Self {
        Self {
            rating,
            motes: Motes {
                peripheral: MoteState {
                    available: rating * 7 + 26,
                    spent: 0,
                },
                personal: MoteState {
                    available: rating * 3 + 10,
                    spent: 0,
                },
                commitments: HashMap::new(),
            },
        }
    }
}

impl<'source> EssenceView<'source> {
    pub(crate) fn new_solar(rating: u8) -> Self {
        Self {
            rating,
            motes: MotesView {
                peripheral: MoteState {
                    available: rating * 7 + 26,
                    spent: 0,
                },
                personal: MoteState {
                    available: rating * 3 + 10,
                    spent: 0,
                },
                commitments: HashMap::new(),
            },
        }
    }
}
