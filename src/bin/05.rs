use aoc_2025::atoi_u64;
use std::cmp::max;

fn main() {
    let file = std::fs::read_to_string("inputs/05.txt").expect("Couldn't read file");
    let (ranges, ingredients) = parse_input(file.as_str());
    let ranges_slice = ranges.as_slice();
    let part_one = part_one(ranges_slice, ingredients);
    let part_two = part_two(ranges_slice);
    println!("{part_one}");
    println!("{part_two}");
}

#[inline(always)]
fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges: Vec<(u64, u64)> = Vec::with_capacity(200);
    let mut ingredients: Vec<u64> = Vec::with_capacity(1000);
    let mut passed_ranges = false;

    for line in input.lines() {
        if line.is_empty() {
            passed_ranges = true
        } else if passed_ranges {
            ingredients.push(atoi_u64(line));
        } else {
            let (start, end) = line.split_once('-').expect("Couldn't parse line as range");
            ranges.push((atoi_u64(start), atoi_u64(end)))
        }
    }

    ranges.sort();

    let merged = ranges.iter().fold(Vec::with_capacity(ranges.len()), |mut acc: Vec<(u64, u64)>, &(l, u)| {
        if let Some(&mut (_, ref mut pu)) = acc.last_mut() {
            if l <= *pu + 1 {
                *pu = (*pu).max(u);
                return acc;
            }
        }
        acc.push((l, u));
        acc
    });

    (merged, ingredients)
}

#[inline(always)]
fn part_one(ranges: &[(u64, u64)], ingredients: Vec<u64>) -> usize {
    ingredients
        .iter()
        .filter(|i| {
            ranges
                .iter()
                .take_while(|(l, _)| l <= i)
                .any(|(_, u)| *i <= u)
        })
        .count()
}

#[inline(always)]
fn part_two(ranges: &[(u64, u64)]) -> u64 {
    let (count, _) = ranges
        .iter()
        .fold((0, 0), |(count, highest), (lower, upper)| {
            if upper < &highest {
                (count, highest)
            } else {
                (count + upper - max(lower, &highest) + 1, *upper + 1)
            }
        });
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

    #[test]
    fn test_parse_input() {
        let (ranges, ingredients) = parse_input(EXAMPLE_INPUT);
        assert_eq!(vec!((3, 5), (10, 20)), ranges);
        assert_eq!(vec!(1, 5, 8, 11, 17, 32), ingredients);
    }

    #[test]
    fn test_part_one() {
        let (ranges, ingredients) = parse_input(EXAMPLE_INPUT);
        assert_eq!(3, part_one(ranges.as_slice(), ingredients));
    }

    #[test]
    fn test_part_two() {
        let (ranges, _) = parse_input(EXAMPLE_INPUT);
        assert_eq!(14, part_two(ranges.as_slice()));
    }

    #[test]
    fn test_part_two_better() {
        assert_eq!(11, part_two(&[(10, 15), (15, 20)]));
        assert_eq!(11, part_two(&[(10, 15), (10, 20)]));
        assert_eq!(11, part_two(&[(10, 20), (10, 20)]));
        assert_eq!(16, part_two(&[(10, 25), (10, 20)]));
        assert_eq!(16, part_two(&[(10, 20), (10, 25)]));
        assert_eq!(11, part_two(&[(10, 15), (15, 20)]));
        assert_eq!(11, part_two(&[(10, 15), (16, 20)]));
        assert_eq!(1, part_two(&[(10, 10)]));
        assert_eq!(1, part_two(&[(10, 10), (10, 10)]));
        assert_eq!(2, part_two(&[(10, 10), (11, 11)]));
    }
}
