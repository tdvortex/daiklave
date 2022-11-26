mod abilities;
mod attributes;
mod merits;
mod mortal;
mod range_bands;
mod weapons;
mod willpower;

pub use crate::abilities::HasAbilities;
pub use crate::mortal::MortalCharacter;

#[cfg(test)]
mod tests {
    use super::MortalCharacter;

    #[test]
    fn default_mortal() {
        let character = MortalCharacter::default();
        dbg!(character);
    }
}
