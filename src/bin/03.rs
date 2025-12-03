fn main() {
    let file = std::fs::read_to_string("inputs/03.txt").expect("Couldn't read file");
    let mut parts = vec!['0'; 12];
    let (part_one, part_two) = file.lines().fold((0, 0), |(part_one, part_two), line| {
        (
            part_one + largest_joltage(line, &mut parts, 2),
            part_two + largest_joltage(line, &mut parts, 12),
        )
    });
    println!("{part_one}");
    println!("{part_two}");
}

fn largest_joltage(bank: &str, parts: &mut Vec<char>, size: usize) -> u64 {
    parts.fill('0');

    let bank_len = bank.len();
    for (i, v) in bank.chars().enumerate() {
        let max_followers_needed = size - 1;
        let remaining_batteries = bank_len - i - 1;
        let parts_to_skip = max_followers_needed
            .checked_sub(remaining_batteries)
            .unwrap_or(0);
        let part_to_replace = parts
            .iter()
            .enumerate()
            .skip(parts_to_skip)
            .find(|(_, p)| p < &&v)
            .map(|(i, _)| i);
        if let Some(part_number) = part_to_replace {
            parts[part_number] = v;
            (part_number + 1..size).for_each(|reset_number| parts[reset_number] = '0')
        }
    }

    parts
        .iter()
        .take(size)
        .map(|c| c.to_digit(10).unwrap() as u64)
        .fold(0u64, |acc, digit| acc * 10 + digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_joltage() {
        let mut parts = vec!['0'; 12];

        assert_eq!(largest_joltage("987654321111111", &mut parts, 2), 98);
        assert_eq!(largest_joltage("811111111111119", &mut parts, 2), 89);
        assert_eq!(largest_joltage("234234234234278", &mut parts, 2), 78);
        assert_eq!(largest_joltage("818181911112111", &mut parts, 2), 92);

        assert_eq!(largest_joltage("987654321111111", &mut parts, 12), 987654321111);
        assert_eq!(largest_joltage("811111111111119", &mut parts, 12), 811111111119);
        assert_eq!(largest_joltage("234234234234278", &mut parts, 12), 434234234278);
        assert_eq!(largest_joltage("818181911112111", &mut parts, 12), 888911112111);
    }
}
