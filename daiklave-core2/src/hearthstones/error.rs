use thiserror::Error;

/// An error related to hearthstone usage.
#[derive(Debug, Error)]
pub enum HearthstoneError {
    /// Can't slot into a full artifact
    #[error("Can't slot there, all hearthstone slots are full")]
    AllSlotsFilled,
    /// Linked hearthstones require a manse
    #[error("Linked hearthstones need a manse to exist")]
    LinkedWithoutManse,
    /// Couldn't find the hearthstone
    #[error("Hearthstone not found")]
    NotFound,
    /// Can't unslot a hearthstone that is already unslotted
    #[error("Hearthstone is not slotted into any artifact")]
    NotSlotted,
    /// Hearthstones must be unique
    #[error("Hearthstones must be unique")]
    UniqueHearthstone,
    /// Wild-Born hearthstones cannot have an associated manse
    #[error("WildBorn hearthstones cannot have manses")]
    WildBornWithManse,
}