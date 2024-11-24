#[cfg(test)]
mod remove_vowels_from_string {
    fn remove_vowels(s: &str) -> String {
        s.chars().filter(|c| !"aeiouAEIOU".contains(*c)).collect()
    }

    #[test]
    fn remove_vowels_from_string1() {
        assert_eq!(remove_vowels("hello"), "hll");
    }

    #[test]
    fn remove_vowels_from_string2() {
        assert_eq!(remove_vowels("world"), "wrld");
    }

    #[test]
    fn remove_vowels_from_string3() {
        assert_eq!(remove_vowels("Rust"), "Rst");
    }

    #[test]
    fn remove_vowels_from_string4() {
        assert_eq!(remove_vowels("Programming"), "Prgrmmng");
    }

    #[test]
    fn remove_vowels_from_string5() {
        assert_eq!(remove_vowels("AEIOU"), "");
    }

    #[test]
    fn remove_vowels_from_string6() {
        assert_eq!(remove_vowels("aeiou"), "");
    }

    #[test]
    fn remove_vowels_from_string7() {
        assert_eq!(remove_vowels("12345"), "12345");
    }

    #[test]
    fn remove_vowels_from_string8() {
        assert_eq!(remove_vowels("XYZ"), "XYZ");
    }

    #[test]
    fn remove_vowels_from_string9() {
        assert_eq!(remove_vowels(""), "");
    }

    #[test]
    fn remove_vowels_from_string10() {
        assert_eq!(
            remove_vowels("bcdfghjklmnpqrstvwxyz"),
            "bcdfghjklmnpqrstvwxyz"
        );
    }
}
