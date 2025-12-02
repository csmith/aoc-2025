use std::io::Write;

fn main() {
    let file = std::fs::read_to_string("inputs/02.txt").expect("Couldn't read file");
    let (part_one, part_two) = solve(file.lines().next().expect("No input?"));
    println!("{part_one}");
    println!("{part_two}");
}

fn solve(input: &str) -> (u64, u64) {
    let mut part_one = 0;
    let mut part_two = 0;

    let mut buffer: Vec<u8> = Vec::with_capacity(20);

    input
        .split(',')
        .map(|pair| {
            let (start, end) = pair.split_once('-').unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .for_each(|(low, high)| {
            for id in low..=high {
                buffer.clear();
                write!(&mut buffer, "{id}").unwrap();
                let len = buffer.len();
                let half_len = len / 2;

                let is_part_one = (len % 2) == 0 && (0..half_len).all(|i| buffer[i] == buffer[half_len + i]);
                let is_part_two = is_part_one
                    || (1..=half_len)
                        .filter(|substr_len| len % substr_len == 0)
                        .any(|substr_len| (substr_len..len).all(|n| buffer[n] == buffer[n % substr_len]));

                if is_part_one {
                    part_one += id
                }
                if is_part_two {
                    part_two += id
                }
            }
        });

    (part_one, part_two)
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
