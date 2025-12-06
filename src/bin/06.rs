fn main() {
    let file = std::fs::read_to_string("inputs/06.txt").expect("Couldn't read file");
    let lines: Vec<Vec<u8>> = file.lines().map(|s| s.bytes().collect()).collect();

    let mut part_one = 0u64;
    let mut part_two = 0u64;
    let mut lefties: Vec<u64> = vec![0u64; 4];
    let mut downies: Vec<u64> = vec![0u64; 4];
    let mut downy_count = 0;
    let mut operator_char = b'?';

    for i in 0..=lines[0].len() {
        let gap = i == lines[0].len() || lines.iter().all(|line| line[i] == b' ');
        if gap {
            let operator = match operator_char {
                b'+' => |v: Vec<u64>| v.iter().sum(),
                b'*' => |v: Vec<u64>| v.iter().fold(1u64, |acc, v| acc * v),
                _ => panic!("Don't know how to {operator_char}"),
            };
            part_one += operator(lefties.clone());
            part_two += operator(downies.iter().take(downy_count).map(|&u| u).collect());
            lefties.fill(0u64);
            downies.fill(0u64);
            downy_count = 0;
            continue
        }

        for x in 0..4 {
            let char = lines[x][i];
            if char != b' ' {
                lefties[x] = lefties[x] * 10 + (char - b'0') as u64;
                downies[downy_count] = downies[downy_count] * 10 + (char - b'0') as u64
            }
        }

        if lines[4][i] != b' ' {
            operator_char = lines[4][i];
        }

        downy_count += 1;
    }

    println!("{part_one}");
    println!("{part_two}");
}
