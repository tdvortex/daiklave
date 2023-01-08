mod no_attunement;
pub(crate) use no_attunement::{WonderNoAttunement, WonderNoAttunementMemo};

use super::WonderId;

/// A Wonder that belongs to the character, and may be attuned or unattuned.
pub struct OwnedWonder<'source>(pub(crate) WonderId, pub(crate) WonderNoAttunement<'source>, pub(crate) Option<u8>);