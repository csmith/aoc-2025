use memmap2::Mmap;
use std::alloc::{Layout, alloc};
use std::fs::File;

fn main() {
    const WIDTH: usize = 141;
    const START: usize = 70;

    let file = File::open("inputs/07.txt").expect("Couldn't open file");
    let bytes: Mmap = unsafe { Mmap::map(&file) }.expect("Couldn't read file");

    let layout = Layout::array::<u64>(WIDTH).unwrap();
    let beams = unsafe { alloc(layout) as *mut u64 };
    unsafe {
        std::ptr::write_bytes(beams, 0, WIDTH);
        *beams.add(START + 1) = 1;
    }

    let mut part_one = 0u64;
    let mut line_offset = (WIDTH + 1) * 2;
    let mut left_bound = START;
    let mut right_bound = START;
    while line_offset < bytes.len() {
        // HERE YE, HERE YE, LET IT BE KNOWN THAT WE WILL BE READING THIS RANGE OF BYTES
        // For some reason this performs better reading the ~full line, trying to limit
        // it to left-bound...right-bound is slower. Not doing this at all is also slower.
        // Computers... How do they work?
        let line = &bytes[line_offset..line_offset + WIDTH];
        let mut i = left_bound;
        while i <= right_bound {
            if line[i] == b'^' {
                unsafe {
                    let value = *beams.add(i + 1);
                    *beams.add(i + 1) = 0;
                    *beams.add(i) += value;
                    *beams.add(i + 2) += value;
                    part_one += (value != 0) as u64;
                    i += 1; // Never another ^ next to us, don't bother checking
                }
            }
            i += 1;
        }
        // Skip the next line as well, there's always a blank one
        line_offset += (WIDTH + 1) * 2;
        left_bound -= 1;
        right_bound += 1;
    }

    // Where does the one come from? Answers on a postcard...
    let part_two: u64 = 1 + (0..WIDTH).map(|i| unsafe { *beams.add(i) }).sum::<u64>();
    println!("{part_one}");
    println!("{part_two}");
}
