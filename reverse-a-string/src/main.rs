mod test;

fn main() {}

fn reverse_a_string(str: &str) -> String {
    str.chars().rev().collect()
}
