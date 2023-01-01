use std::collections::HashMap;

use super::{mote_pool::MotePool, motes_view::MotesView, EssenceMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EssenceView<'source> {
    pub(crate) rating: u8,
    pub(crate) motes: MotesView<'source>,
}

impl<'source> EssenceView<'source> {
    pub(crate) fn as_memo(&self) -> EssenceMemo {
        EssenceMemo::new(self.rating, self.motes.as_memo())
    }

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
            motes: MotesView::new(
                MotePool::new(rating * 7 + 26, 0),
                MotePool::new(rating * 3 + 10, 0),
                HashMap::new(),
            ),
        }
    }
}
