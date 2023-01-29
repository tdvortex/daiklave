use std::{collections::HashMap, num::NonZeroU8};

use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::NonnaturalArtifactWeaponMemo, mundane::NonnaturalMundaneWeaponMemo,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltUnequippedWeaponsMemo {
    pub mundane: HashMap<String, (NonnaturalMundaneWeaponMemo, NonZeroU8)>,
    pub artifact: HashMap<String, NonnaturalArtifactWeaponMemo>,
}
