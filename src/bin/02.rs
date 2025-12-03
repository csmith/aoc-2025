fn main() {
    let file = std::fs::read_to_string("inputs/02.txt").expect("Couldn't read file");
    let (part_one, part_two) = solve(file.lines().next().expect("No input?"));
    println!("{part_one}");
    println!("{part_two}");
}

fn solve(input: &str) -> (u64, u64) {
    let mut part_one = 0;
    let mut part_two = 0;

    // npm install --save is-even
    let even: [bool; 12] = [
        false, false, true, false, true, false, true, false, true, false, true, false,
    ];

    let dividers: [Vec<usize>; 12] = [
        vec![],
        vec![],
        vec![],
        vec![1],
        vec![1],
        vec![1],
        vec![1, 2],
        vec![1],
        vec![1, 2],
        vec![1, 3],
        vec![1, 2],
        vec![1],
    ];

    let mut id: [u8; 20] = [0; 20];
    let mut target: [u8; 20] = [0; 20];

    input
        .split(',')
        .map(|pair| {
            let (start, end) = pair.split_once('-').unwrap();
            (start.as_bytes(), end.as_bytes())
        })
        .for_each(|(low, high)| {
            // To make it easier to manually increment the byte arrays, store
            // them reversed (so we don't have to shift when overflowing from
            // 999 to 1000, say). This makes no difference to the actual checks.
            target[..high.len()].copy_from_slice(high);
            target[..high.len()].reverse();
            let target_len = high.len();

            id[..low.len()].copy_from_slice(low);
            id[..low.len()].reverse();
            let mut len = low.len();
            let mut half_len = len / 2;
            loop {
                let is_part_one = even[len] && (0..half_len).all(|i| id[i] == id[half_len + i]);
                let is_part_two = is_part_one
                    || dividers[len].iter().any(|substr_len| {
                        let mut idx = 0;
                        (*substr_len..len).all(|n| {
                            // Manually track the original index to avoid doing modulo ops
                            let result = id[n] == id[idx];
                            idx += 1;
                            if idx == *substr_len {
                                idx = 0;
                            }
                            result
                        })
                    });

                if is_part_two {
                    let val = atoi_u64_bytes(id, len);
                    part_two += val;

                    if is_part_one {
                        part_one += val;
                    }
                }

                if len == target_len && id[..len] == target[..len] {
                    break;
                }

                for i in 0..len {
                    if id[i] != b'9' {
                        id[i] += 1;
                        break;
                    } else {
                        id[i] = b'0';
                        if i == len - 1 {
                            id[len] = b'1';
                            len += 1;
                            half_len = len / 2;
                        }
                    }
                }
            }
        });

    (part_one, part_two)
}

fn atoi_u64_bytes(bytes: [u8; 20], len: usize) -> u64 {
    let mut acc = 0u64;
    for i in (0..len).rev() {
        acc = acc * 10 + (bytes[i] - b'0') as u64
    }
    acc
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
