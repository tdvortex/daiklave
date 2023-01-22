use serde::{Deserialize, Serialize};

/// Published Exalted 3e books.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Book {
    /// The core rulebook.
    CoreRulebook,
    /// Arms of the Chosen, featuring a variety of Artifacts and Hearthstones.
    ArmsOfTheChosen,
}
