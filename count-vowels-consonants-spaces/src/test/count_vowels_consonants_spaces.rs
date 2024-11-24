#[cfg(test)]
mod count_vowels_consonants_spaces {
    use crate::count_vowels_consonants_spaces;

    #[test]
    fn count_vowels_consonants_spaces1() {
        let result = count_vowels_consonants_spaces("Hello World");
        assert_eq!(result, (3, 7, 1));
    }
    #[test]
    fn count_vowels_consonants_spaces2() {
        let result = count_vowels_consonants_spaces("The quick brown fox jumps over the lazy dog.");
        assert_eq!(result, (11, 24, 8));
    }
    #[test]
    fn count_vowels_consonants_spaces3() {
        let result = count_vowels_consonants_spaces("A E I O U a e i o u");
        assert_eq!(result, (10, 0, 9));
    }
    #[test]
    fn count_vowels_consonants_spaces4() {
        let result = count_vowels_consonants_spaces("1234567890");
        assert_eq!(result, (0, 0, 0));
    }
}
