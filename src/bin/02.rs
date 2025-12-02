fn main() {
    let file = std::fs::read_to_string("inputs/02.txt").expect("Couldn't read file");
    let (part_one, part_two) = solve(file.lines().next().expect("No input?"));
    println!("{part_one}");
    println!("{part_two}");
}

fn solve(input: &str) -> (u64, u64) {
    input
        .split(',')
        .map(|pair| {
            let (start, end) = pair.split_once('-').unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .flat_map(|(low, high)| low..=high)
        .map(|id| {
            let bytes = id.to_string().into_bytes();
            let len = bytes.len();

            let is_part_one = (len % 2) == 0 && bytes[0..len / 2] == bytes[len / 2..];
            let is_part_two = is_part_one
                || (1..=len / 2)
                    .filter(|i| len % i == 0)
                    .any(|i| (i..len).all(|n| bytes[n] == bytes[n % i]));

            (
                if is_part_one { id } else { 0 },
                if is_part_two { id } else { 0 },
            )
        })
        .fold((0, 0), |(a1, a2), (b1, b2)| (a1 + b1, a2 + b2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_example() {
        assert_eq!(solve(EXAMPLE_INPUT), (1227775554, 4174379265));
    }
}
