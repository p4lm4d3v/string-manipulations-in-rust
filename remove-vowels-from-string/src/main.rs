mod test;

fn main() {}

fn remove_vowels_from_string(str: String) -> String {
    str.chars().filter(|c| !"aeiouAEIOU".contains(*c)).collect()
}
