#[cfg(test)]
mod sum_of_numbers_in_a_string {
    use crate::sum_of_numbers_in_a_string;

    #[test]
    fn sum_of_number_in_a_string1() {
        let result = sum_of_numbers_in_a_string(String::from("12a34"));
        assert_eq!(result, 10);
    }
    #[test]
    fn sum_of_number_in_a_string2() {
        let result = sum_of_numbers_in_a_string(String::from("abc123xyz"));
        assert_eq!(result, 6);
    }
    #[test]
    fn sum_of_number_in_a_string3() {
        let result = sum_of_numbers_in_a_string(String::from("noNumbersHere"));
        assert_eq!(result, 0);
    }
    #[test]
    fn sum_of_number_in_a_string4() {
        let result = sum_of_numbers_in_a_string(String::from("456def789"));
        assert_eq!(result, 39);
    }
    #[test]
    fn sum_of_number_in_a_string5() {
        let result = sum_of_numbers_in_a_string(String::from("1a2b3c4d5e"));
        assert_eq!(result, 15);
    }
    #[test]
    fn sum_of_number_in_a_string6() {
        let result = sum_of_numbers_in_a_string(String::from("0000"));
        assert_eq!(result, 0);
    }
    #[test]
    fn sum_of_number_in_a_string7() {
        let result = sum_of_numbers_in_a_string(String::from("1234567890"));
        assert_eq!(result, 45);
    }
    #[test]
    fn sum_of_number_in_a_string8() {
        let result = sum_of_numbers_in_a_string(String::from("a1b2c3d4e5f6g7h8i9j0"));
        assert_eq!(result, 45);
    }
    #[test]
    fn sum_of_number_in_a_string9() {
        let result = sum_of_numbers_in_a_string(String::from("123abc456"));
        assert_eq!(result, 21);
    }
    #[test]
    fn sum_of_number_in_a_string10() {
        let result = sum_of_numbers_in_a_string(String::from("9876543210"));
        assert_eq!(result, 45);
    }
}
