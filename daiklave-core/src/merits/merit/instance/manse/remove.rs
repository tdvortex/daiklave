use super::ManseName;

/// A mutation to remove a manse from a character. Will also remove the
/// associated demense and hearthstone.
pub struct RemoveManse(pub ManseName);

impl RemoveManse {
    /// The name of the manse to remove.
    pub fn name(name: impl Into<ManseName>) -> Self {
        Self(name.into())
    }
}
