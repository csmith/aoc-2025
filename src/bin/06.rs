use aoc_2025::atoi_u64;

fn main() {
    let file = std::fs::read_to_string("inputs/06.txt").expect("Couldn't read file");
    let mut bits: Vec<Vec<&str>> = Vec::new();
    file.lines().for_each(|l| {
        l.split_whitespace().enumerate().for_each(|(i, bit)| {
            if bits.len() <= i {
                bits.push(Vec::new())
            }
            bits[i].push(bit);
        })
    });

    let (part_one, part_two): (u64, u64) =
        bits.iter()
            .fold((0u64, 0u64), |(part_one, part_two), parts| {
                let op_str = parts.last().unwrap();
                let op = match *op_str {
                    "*" => |values: Vec<&str>| {
                        values
                            .iter()
                            .map(|&v| atoi_u64(v))
                            .fold(1u64, |acc, v| acc * v)
                    },
                    "+" => |values: Vec<&str>| {
                        values
                            .iter()
                            .map(|&v| atoi_u64(v))
                            .fold(0u64, |acc, v| acc + v)
                    },
                    _ => panic!("Unknown op {op_str}"),
                };

                // "What's the design goal for this programming language?"
                // "IDK, how about making trivial string manipulation rage-inducing for some reason?"
                let terms: Vec<&str> = parts.iter().take(parts.len() - 1).copied().collect();
                let max_length = terms.iter().map(|x| x.len()).max().unwrap();
                // This is wrong, they shouldn't be padded, it needs to be aware of the arbitrary whitespace in the input,
                // which means rewriting how the parsing works, but this is not fun in the slightest.
                let padded_terms: Vec<String> = terms.iter().map(|&x| format!("{:>width$}", x, width=max_length)).collect();
                let transposed_terms: Vec<String> = padded_terms.iter().fold(vec![String::new(); max_length], |mut vec, term| {
                    term.bytes().enumerate().for_each(|(i, c)| {
                        vec[i].push(c as char);
                    });
                    vec
                });
                let transposed_terms_stripped: Vec<&str> = transposed_terms.iter().map(|s| s.trim()).collect();

                (
                    part_one + op(terms),
                    part_two + op(transposed_terms_stripped),
                )
            });
    println!("{part_one}");
    println!("{part_two}");
}
