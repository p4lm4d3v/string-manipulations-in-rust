mod test;

fn main() {}

fn is_palindrome(str: &str) -> bool {
    str == str
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .iter()
        .map(|c| c.as_str())
        .rev()
        .collect::<String>()
}
