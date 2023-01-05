use serde::{Serialize, Deserialize};

use crate::weapons::{artifact::ArtifactWeaponMemo, ArtifactWeapon};

/// A magical item owned by a character.
pub enum Artifact<'source> {
    /// An artifact weapon like a daiklave or direlash.
    Weapon(ArtifactWeapon<'source>)
}

impl<'source> Artifact<'source> {
    /// Creates an owned copy of the artifact.
    pub fn as_memo(&self) -> ArtifactMemo {
        match self {
            Artifact::Weapon(view) => ArtifactMemo::Weapon(view.as_memo())
        }
    }
}

/// An owned copy of an Artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactMemo {
    /// An artifact weapon.
    Weapon(ArtifactWeaponMemo)
}

impl<'source> ArtifactMemo {
    /// Uses the Artifact as a source and copies all Copy values and derefs
    /// String to &'source str
    pub fn as_ref(&'source self) -> Artifact<'source> {
        match self {
            ArtifactMemo::Weapon(memo) => Artifact::Weapon(memo.as_ref()),
        }
    }
}

/// One of the Magic Materials used to construct Artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum MagicMaterial {
    /// Orichalcum, a gold metal. Only Solars are resonant with it. Sidereals,
    /// Liminals, and Getimians are dissonant.
    Orichalcum,
    /// Starmetal, a rare iridescent metal. Solars, Sidereals, and Getimians
    /// are resonant. 
    Starmetal,
    /// The forged souls of the dead. Solars and Abyssals are resonant. 
    /// Sidereals, Dragon-Blooded, and Getimians are dissonant.
    Soulsteel,
    /// A fluid, shifting silver. Solars and Lunars are resonant. Sidereals,
    /// Liminals, and Getimians are dissonant.
    Moonsilver,
    /// Jade with fire essence. Solars and Dragon-Blooded are resonant. 
    /// Sidereals, Liminals, and Getimians are dissonant.
    RedJade,
    /// Jade with air essence. Solars and Dragon-Blooded are resonant. 
    /// Sidereals, Liminals, and Getimians are dissonant.
    BlueJade,
    /// Jade with wood essence. Solars and Dragon-Blooded are resonant. 
    /// Sidereals, Liminals, and Getimians are dissonant.
    GreenJade,
    /// Jade with water essence. Solars and Dragon-Blooded are resonant. 
    /// Sidereals, Liminals, and Getimians are dissonant.
    BlackJade,
    /// Jade with earth essence. Solars and Dragon-Blooded are resonant. 
    /// Sidereals, Liminals, and Getimians are dissonant.
    WhiteJade,
}