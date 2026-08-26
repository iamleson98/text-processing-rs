//! Word tagger for Vietnamese.
//!
//! Converts spoken Vietnamese letter sequences to written form:
//! - "a bê xê" → "ABC"
//! - "u ét xê" → "USA"
//!
//! Vietnamese letter names (alphabet):
//! A=a, B=bê, C=xê, D=dê, Đ=đê, E=e, G=gờ, H=hát, I=i, K=ca, L=lờ, M=mờ,
//! N=nờ, O=o, P=pê, Q=quy, R=rờ, S=ét, T=tê, U=u, V=vê, X=ích, Y=i-cờ-rết,
//! Z=dét, F=ép, J=gi, W=vê-kép.

/// Parse a spoken Vietnamese letter sequence to its uppercase written form.
pub fn parse(input: &str) -> Option<String> {
    let input_lower = input.trim().to_lowercase();
    let tokens: Vec<&str> = input_lower.split_whitespace().collect();

    // Need at least 2 letters to be considered a sequence.
    if tokens.len() < 2 {
        return None;
    }

    let mut letters = String::new();
    for token in &tokens {
        let letter = parse_letter(token)?;
        letters.push_str(&letter);
    }

    Some(letters)
}

/// Parse a single Vietnamese letter word to its uppercase letter.
fn parse_letter(word: &str) -> Option<String> {
    let letter_map = [
        ("a", "A"),
        ("bê", "B"),
        ("bờ", "B"),
        ("xê", "C"),
        ("cờ", "C"),
        ("dê", "D"),
        ("đê", "Đ"),
        ("e", "E"),
        ("ép", "F"),
        ("gờ", "G"),
        ("hát", "H"),
        ("hạch", "H"),
        ("i", "I"),
        ("gi", "J"),
        ("ca", "K"),
        ("kê", "K"),
        ("lờ", "L"),
        ("el", "L"),
        ("mờ", "M"),
        ("em", "M"),
        ("nờ", "N"),
        ("en", "N"),
        ("o", "O"),
        ("pê", "P"),
        ("quy", "Q"),
        ("quờ", "Q"),
        ("rờ", "R"),
        ("ét", "S"),
        ("ét xì", "S"),
        ("tê", "T"),
        ("u", "U"),
        ("vê", "V"),
        ("vê kép", "W"),
        ("vê đúp", "W"),
        ("ích", "X"),
        ("i cờ rết", "Y"),
        ("i gờ rếch", "Y"),
        ("dét", "Z"),
    ];

    for (spoken, letter) in &letter_map {
        if word == *spoken {
            return Some(letter.to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_sequence() {
        assert_eq!(parse("a bê xê"), Some("ABC".to_string()));
    }

    #[test]
    fn test_longer_sequence() {
        assert_eq!(parse("u ét a"), Some("USA".to_string()));
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("a"), None); // Single letter
        assert_eq!(parse("hello world"), None); // Not letters
        assert_eq!(parse(""), None);
    }
}
