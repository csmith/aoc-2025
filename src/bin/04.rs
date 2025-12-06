use memmap2::Mmap;
use std::alloc::{Layout, alloc};
use std::fs::File;

const ROLL_PRESENT: i8 = 0b00100000;
const ROLL_NEWLY_REMOVABLE: i8 = 0b00100011;

fn main() {
    let file = File::open("inputs/04.txt").expect("Couldn't open file");
    let bytes: Mmap = unsafe { Mmap::map(&file) }.expect("Couldn't read file");
    let size = bytes.len().isqrt();
    let padded_size = size + 2;
    let total_size = padded_size * padded_size;

    let layout = Layout::array::<i8>(total_size).unwrap();
    let memory = unsafe { alloc(layout) as *mut i8 };
    let mut pending_removals: Vec<usize> = Vec::with_capacity(1750);

    unsafe {
        std::ptr::write_bytes(memory, 0, total_size);
    }

    let mut offset = padded_size + 1;
    let mut x = 0;
    for b in bytes.iter() {
        match b {
            b'@' => {
                unsafe {
                    *memory.add(offset + x) |= ROLL_PRESENT;
                }
                x += 1;
                if x == size {
                    offset += padded_size;
                    x = 0;
                }
            }
            b'.' => {
                x += 1;
                if x == size {
                    offset += padded_size;
                    x = 0;
                }
            }
            _ => {}
        }
    }

    // First pass: mark all the rolls with <4 neighbours, which also happens
    // to get us the part one answer
    for index in 0..total_size {
        unsafe {
            let value = *memory.add(index);
            if roll_present(value) {
                let neighbours = roll_present(*memory.add(index - 1)) as i8
                    + roll_present(*memory.add(index + 1)) as i8
                    + roll_present(*memory.add(index - 1 - padded_size)) as i8
                    + roll_present(*memory.add(index + 1 - padded_size)) as i8
                    + roll_present(*memory.add(index - padded_size)) as i8
                    + roll_present(*memory.add(index - 1 + padded_size)) as i8
                    + roll_present(*memory.add(index + 1 + padded_size)) as i8
                    + roll_present(*memory.add(index + padded_size)) as i8;
                *memory.add(index) = value + neighbours;
                if neighbours < 4 {
                    pending_removals.push(index);
                }
            }
        }
    }

    let part_one = pending_removals.len();
    println!("{part_one}");

    let mut part_two = 0;

    // Remainder: for each pending removal, decrement all the neighbours and
    // see if they're now eligible for removal
    while let Some(index) = pending_removals.pop() {
        notify_neighbour_of_removal(memory, &mut pending_removals, index - 1);
        notify_neighbour_of_removal(memory, &mut pending_removals, index + 1);
        notify_neighbour_of_removal(memory, &mut pending_removals, index - 1 - padded_size);
        notify_neighbour_of_removal(memory, &mut pending_removals, index + 1 - padded_size);
        notify_neighbour_of_removal(memory, &mut pending_removals, index - padded_size);
        notify_neighbour_of_removal(memory, &mut pending_removals, index - 1 + padded_size);
        notify_neighbour_of_removal(memory, &mut pending_removals, index + 1 + padded_size);
        notify_neighbour_of_removal(memory, &mut pending_removals, index + padded_size);
        part_two += 1;
    }

    println!("{part_two}")
}

#[inline(always)]
fn notify_neighbour_of_removal(memory: *mut i8, pending_removals: &mut Vec<usize>, index: usize) {
    unsafe {
        let pointer = memory.add(index);
        *pointer -= 1;
        if *pointer == ROLL_NEWLY_REMOVABLE {
            pending_removals.push(index);
        }
    }
}

#[inline(always)]
fn roll_present(value: i8) -> bool {
    value & ROLL_PRESENT == ROLL_PRESENT
}
