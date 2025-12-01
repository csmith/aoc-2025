fn main() {
    let file = std::fs::read_to_string("inputs/01.txt").unwrap();

    let mut dial: i32 = 50;
    let mut exact_zeroes: u32 = 0;
    let mut passing_zeroes: u32 = 0;

    for l in file.lines() {
        let old_dial = dial;
        let direction = l.chars().next().unwrap();
        let distance: i32 = l[1..].parse().unwrap();

        match direction {
            'L' => dial -= distance,
            'R' => dial += distance,
            _ => panic!("Unexpected direction '{}'", direction)
        }

        // There are SO. MANY. edge cases trying to do this nicely.
        //
        //  5, 0,  5 ==> needs to count once, but isn't counted by checking floor(old/100) vs floor(new/100)
        // -5, 0, -5 ==> needs to count once, but is double counted by that
        //
        // Fiddle with the min and max so we don't ever count starting/ending
        // at zero as passing it. We'll just add the part 1 answer to cope with
        // those
        //
        // Maybe there's a better solution, but after the fourth time I got the
        // right answer for the example input, and the wrong answer for my own
        // I lost patience.
        let mut min = old_dial.min(dial);
        let mut max = old_dial.max(dial);
        if min % 100 == 0 {
            min += 1
        }
        if max % 100 == 0 {
            max -= 1
        }
        passing_zeroes += min.div_euclid(100).abs_diff(max.div_euclid(100));

        dial = ((dial % 100) + 100) % 100;
        if dial == 0 {
            exact_zeroes += 1;
        }
    }

    println!("{}", exact_zeroes);
    println!("{}", exact_zeroes + passing_zeroes);
}
