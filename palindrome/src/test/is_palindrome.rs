#[cfg(test)]
mod is_palindrome {
    use crate::is_palindrome;

    #[test]
    fn is_palindrome1() {
        let result = is_palindrome("anavolimilovana");
        assert!(result);
    }

    #[test]
    fn is_palindrome2() {
        let result = is_palindrome("abba");
        assert!(result);
    }

    #[test]
    fn is_palindrome3() {
        let result = is_palindrome("str");
        assert!(!result);
    }
}
