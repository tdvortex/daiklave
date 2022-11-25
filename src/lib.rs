mod character;
mod mortal;
mod range_bands;

#[cfg(test)]
mod tests {
    #[test]
    fn default_mortal() {
        use crate::mortal::MortalCharacter;
        let character = MortalCharacter::default();
        dbg!(character);
    }
}
