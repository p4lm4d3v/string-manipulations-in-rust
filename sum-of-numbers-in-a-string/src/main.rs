mod test;

fn main() {}

pub fn sum_of_numbers_in_a_string(str: String) -> isize {
    let mut sum = 0;

    for c in str.chars() {
        if let Some(n) = c.to_digit(10) {
            sum += n as isize;
        }
    }

    sum
}
