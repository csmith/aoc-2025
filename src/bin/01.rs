const DIAL_SIZE: i32 = 100;
const INITIAL_POSITION: i32 = 50;

fn main() {
    let file = std::fs::read_to_string("inputs/01.txt").expect("Couldn't read file");

    let (_, part_one, part_two) = file.lines().map(parse_line).fold(
        (INITIAL_POSITION, 0, 0),
        |(position, exact_zeroes, passing_zeroes), movement| {
            let new_position = rotate(position, movement);
            let passes = count_passes(position, new_position);
            let at_zero = u32::from(new_position % DIAL_SIZE == 0);

            (
                new_position,
                exact_zeroes + at_zero,
                passing_zeroes + passes + at_zero,
            )
        },
    );

    println!("{part_one}");
    println!("{part_two}");
}

fn rotate(position: i32, movement: (bool, i32)) -> i32 {
    let (clockwise, distance) = movement;
    position + distance * if clockwise { 1 } else { -1 }
}

fn parse_line(line: &str) -> (bool, i32) {
    let mut chars = line.chars();
    let clockwise = match chars.next().expect("No direction") {
        'R' => true,
        'L' => false,
        other => panic!("Unknown direction '{other}'"),
    };
    let distance: i32 = chars.as_str().parse().expect("Invalid distance");

    (clockwise, distance)
}

fn count_passes(old_dial_position: i32, new_dial_position: i32) -> u32 {
    // We don't want to count when we start/end on a multiple of 100, as it'll
    // cause fencepost errors. To do this, we create a half-open interval by
    // adjusting one of the positions. It's a bit fiddly as the way we
    // handle 0 isn't symmetrical (floor(0/100) == floor(1/100) != floor(-1/100)),
    // so the adjustment depends on the direction of rotation.
    let clockwise = i32::from(new_dial_position > old_dial_position);
    let start = old_dial_position + clockwise - 1; // -1 if anti-clockwise
    let end = new_dial_position - clockwise; // -1 if clockwise
    start
        .div_euclid(DIAL_SIZE)
        .abs_diff(end.div_euclid(DIAL_SIZE))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_passes() {
        assert_eq!(count_passes(1, 99), 0);
        assert_eq!(count_passes(-1, -99), 0);
        assert_eq!(count_passes(0, 1), 0);
        assert_eq!(count_passes(1, 0), 0);
        assert_eq!(count_passes(-1, 0), 0);
        assert_eq!(count_passes(0, -1), 0);
        assert_eq!(count_passes(0, 100), 0);
        assert_eq!(count_passes(0, 200), 1);
        assert_eq!(count_passes(-100, 200), 2);
        assert_eq!(count_passes(-1, 1), 1);
        assert_eq!(count_passes(-100, 0), 0);
    }
}
