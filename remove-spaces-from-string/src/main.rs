mod test;

fn main() {}

fn remove_spaces_from_string(str: String) -> String {
    str.chars().filter(|c| *c != ' ').collect()
}
