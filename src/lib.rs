use std::borrow::Cow;

// Removes Arabic diacritics from a string.
///
/// Reference: https://www.unicode.org/charts/PDF/U0600.pdf
/// ```
/// let word = tashkil::remove("تَشْكِيل");
/// assert_eq!("تشكيل", word);
/// ```
///
pub fn remove(string: &str) -> Cow<'_, str> {
    string.chars().filter(|&c| !is_diacritic(c)).collect()
}

/// Returns true if the character is a diacritic
fn is_diacritic(c: char) -> bool {
    matches!(c, '\u{0610}'..='\u{0614}') // Honorifics
    || matches!(c, '\u{0615}') // Quranic annotation sign
    || matches!(c, '\u{0617}'..='\u{061A}') // Quranic annotation signs
    || matches!(c, '\u{061C}') // Format character
    || matches!(c, '\u{0640}') // Tatwil
    || matches!(c, '\u{064B}'..='\u{0652}') // Tashkil
    || matches!(c, '\u{0653}'..='\u{0655}') // Combining madda and hamza
    || matches!(c, '\u{0656}'..='\u{065F}') // Other combining marks
    || matches!(c, '\u{0660}') // Tashkil
    || matches!(c, '\u{06D6}'..='\u{06ED}') // Quranic annotation signs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_tashkil() {
        let removed = remove("تَشْدِيد");
        assert_eq!("تشديد", removed);
    }
}
