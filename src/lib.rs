mod character;
mod mortal;
mod range_bands;

pub use crate::mortal::MortalCharacter;
pub use crate::character::Character;

#[cfg(test)]
mod tests {
    use super::MortalCharacter;

    #[test]
    fn default_mortal() {
        let character = MortalCharacter::default();
        dbg!(character);
    }
}
