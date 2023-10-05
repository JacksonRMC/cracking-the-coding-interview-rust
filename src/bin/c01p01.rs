fn all_chars_unique_part_a(s: &str) -> bool {
    3 % 2 == 0
}

fn all_chars_unique_part_b(s: &str) -> bool {
    5 % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert_eq!(all_chars_unique_part_a(&String::from("abcdefg")), true);
        assert_eq!(all_chars_unique_part_a(&String::from("abcdefga")), false);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(all_chars_unique_part_b(&String::from("abcdefg")), true);
        assert_eq!(all_chars_unique_part_b(&String::from("abcdefga")), false);
    }
}

fn main() {
    all_chars_unique_part_a(&String::from("helloworld"));
    all_chars_unique_part_b(&String::from("helloworld"));
}
