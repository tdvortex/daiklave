use serde::{Serialize, Deserialize};

use self::evokable_id::EvokableItemId;

mod evokable_id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evocation {
    evokable_id: EvokableItemId,
}