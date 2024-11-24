#[cfg(test)]
mod reverse_a_string {
    use crate::reverse_a_string;

    #[test]
    fn reverse_string_test1() {
        assert_eq!(reverse_a_string("hello"), "olleh");
    }

    #[test]
    fn reverse_string_test2() {
        assert_eq!(reverse_a_string("world"), "dlrow");
    }

    #[test]
    fn reverse_string_test3() {
        assert_eq!(reverse_a_string("Rust"), "tsuR");
    }

    #[test]
    fn reverse_string_test4() {
        assert_eq!(reverse_a_string("Programming"), "gnimmargorP");
    }

    #[test]
    fn reverse_string_test5() {
        assert_eq!(reverse_a_string("AEIOU"), "UOIEA");
    }

    #[test]
    fn reverse_string_test6() {
        assert_eq!(reverse_a_string("aeiou"), "uoiea");
    }

    #[test]
    fn reverse_string_test7() {
        assert_eq!(reverse_a_string("12345"), "54321");
    }

    #[test]
    fn reverse_string_test8() {
        assert_eq!(reverse_a_string("XYZ"), "ZYX");
    }

    #[test]
    fn reverse_string_test9() {
        assert_eq!(reverse_a_string(""), "");
    }

    #[test]
    fn reverse_string_test10() {
        assert_eq!(
            reverse_a_string("abcdefghijklmnopqrstuvwxyz"),
            "zyxwvutsrqponmlkjihgfedcba"
        );
    }
}
