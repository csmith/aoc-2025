use memmap2::Mmap;
use std::alloc::{Layout, alloc};
use std::fs::File;

fn main() {
    let file = File::open("inputs/07.txt").expect("Couldn't open file");
    let bytes: Mmap = unsafe { Mmap::map(&file) }.expect("Couldn't read file");
    let lines: Vec<&[u8]> = bytes.split(|&b| b == b'\n').collect();
    let width = lines[0].len();

    let layout = Layout::array::<u64>(width + 2).unwrap();
    let beams = unsafe { alloc(layout) as *mut u64 };
    unsafe {
        std::ptr::write_bytes(beams, 0, width);
    }

    let mut part_one = 0u64;

    lines.iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, v)| v)
        .for_each(|l| {
        l.iter().enumerate().for_each(|(i, v)| match v {
            b'S' => unsafe { *beams.add(i + 1) += 1 },
            b'^' => unsafe {
                let value = *beams.add(i + 1);
                if value != 0 {
                    *beams.add(i + 1) = 0;
                    *beams.add(i) += value;
                    *beams.add(i + 2) += value;
                    part_one += 1;
                }
            },
            _ => {}
        });
    });

    // Where does the one come from? Answers on a postcard...
    let part_two: u64 = 1 + (0..width).map(|i| unsafe { *beams.add(i) }).sum::<u64>();
    println!("{part_one}");
    println!("{part_two}");
}
