use serde::{Deserialize, Serialize};

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
