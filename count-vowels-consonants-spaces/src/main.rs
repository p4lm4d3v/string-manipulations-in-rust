mod test;

fn main() {}

fn count_vowels_consonants_spaces(str: &str) -> (usize, usize, usize) {
    let str = str.to_lowercase();

    let mut vowels = 0;
    let mut consonants = 0;
    let mut spaces = 0;

    for s in str.chars() {
        let s = s.to_string();
        let s = s.as_str();
        match &s {
            &" " => spaces += 1,
            s if is_vowel(s) => vowels += 1,
            s if is_consonant(s) => consonants += 1,
            _ => (),
        }
    }

    (vowels, consonants, spaces)
}

fn is_vowel(str: &str) -> bool {
    vec!["a", "e", "i", "o", "u"].contains(&str)
}

fn is_consonant(str: &str) -> bool {
    vec![
        "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w",
        "x", "y", "z",
    ]
    .contains(&str)
}
