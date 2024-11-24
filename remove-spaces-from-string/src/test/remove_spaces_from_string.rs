#[cfg(test)]
mod remove_spaces_from_string {
    use crate::remove_spaces_from_string;

    #[test]
    fn remove_spaces_from_string1() {
        assert_eq!(
            remove_spaces_from_string("hello world".to_string()),
            "helloworld"
        );
    }

    #[test]
    fn remove_spaces_from_string2() {
        assert_eq!(remove_spaces_from_string(" Rust ".to_string()), "Rust");
    }

    #[test]
    fn remove_spaces_from_string3() {
        assert_eq!(
            remove_spaces_from_string("  Programming  ".to_string()),
            "Programming"
        );
    }

    #[test]
    fn remove_spaces_from_string4() {
        assert_eq!(remove_spaces_from_string("a e i o u".to_string()), "aeiou");
    }

    #[test]
    fn remove_spaces_from_string5() {
        assert_eq!(remove_spaces_from_string("A E I O U".to_string()), "AEIOU");
    }

    #[test]
    fn remove_spaces_from_string6() {
        assert_eq!(remove_spaces_from_string("1 2 3 4 5".to_string()), "12345");
    }

    #[test]
    fn remove_spaces_from_string7() {
        assert_eq!(remove_spaces_from_string("XYZ".to_string()), "XYZ");
    }

    #[test]
    fn remove_spaces_from_string8() {
        assert_eq!(remove_spaces_from_string("".to_string()), "");
    }

    #[test]
    fn remove_spaces_from_string9() {
        assert_eq!(remove_spaces_from_string("   ".to_string()), "");
    }

    #[test]
    fn remove_spaces_from_string10() {
        assert_eq!(
            remove_spaces_from_string("bcd fgh jkl mnp qrs tuv wxy z".to_string()),
            "bcdfghjklmnpqrstuvwxyz"
        );
    }
}
