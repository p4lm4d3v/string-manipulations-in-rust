#[cfg(test)]
mod ascii_value_of_character {
    use crate::ascii_value_of_character;

    #[test]
    fn ascii_value_of_character1() {
        assert_eq!(ascii_value_of_character('a'), 97);
    }

    #[test]
    fn ascii_value_of_character2() {
        assert_eq!(ascii_value_of_character('z'), 122);
    }

    #[test]
    fn ascii_value_of_character3() {
        assert_eq!(ascii_value_of_character('A'), 65);
    }

    #[test]
    fn ascii_value_of_character4() {
        assert_eq!(ascii_value_of_character('Z'), 90);
    }

    #[test]
    fn ascii_value_of_character5() {
        assert_eq!(ascii_value_of_character('0'), 48);
    }

    #[test]
    fn ascii_value_of_character6() {
        assert_eq!(ascii_value_of_character('9'), 57);
    }

    #[test]
    fn ascii_value_of_character7() {
        assert_eq!(ascii_value_of_character(' '), 32);
    }

    #[test]
    fn ascii_value_of_character8() {
        assert_eq!(ascii_value_of_character('\n'), 10);
    }

    #[test]
    fn ascii_value_of_character9() {
        assert_eq!(ascii_value_of_character('\t'), 9);
    }

    #[test]
    fn ascii_value_of_character10() {
        assert_eq!(ascii_value_of_character('!'), 33);
    }
}
