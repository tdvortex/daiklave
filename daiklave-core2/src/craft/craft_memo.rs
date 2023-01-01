use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{abilities::AbilityMemo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct CraftMemo(HashMap<String, AbilityMemo>);
