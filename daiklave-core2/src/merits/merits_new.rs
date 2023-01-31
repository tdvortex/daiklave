use crate::Character;

use super::merit_new::{Merit};

pub struct Merits<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> Merits<'view, 'source> {
    pub fn iter(&self) -> impl Iterator<Item = Merit<'source>> {
        todo!()
    }
}