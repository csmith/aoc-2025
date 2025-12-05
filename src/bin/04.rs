use std::alloc::{alloc, Layout};
use memmap2::Mmap;
use std::fs::File;

fn main() {
    let file = File::open("inputs/04.txt").expect("Couldn't open file");
    let bytes: Mmap = unsafe { Mmap::map(&file) }
        .expect("Couldn't read file");
    let size = bytes.len().isqrt();
    let padded_size = size+2;
    let total_size = padded_size * padded_size;

    let layout = Layout::array::<bool>(total_size * 2).unwrap();
    let memory = unsafe { alloc(layout) as *mut bool };
    let mut rolls: Vec<usize> = Vec::with_capacity(15000);

    unsafe {
        std::ptr::write_bytes(memory, 0, total_size * 2);
    }

    let mut roll_count = 0;
    let mut offset = padded_size + 1;
    let mut x = 0;
    for b in bytes.iter() {
        match b {
            b'@' => {
                rolls.push(offset+x);
                roll_count += 1;
                unsafe {
                    *memory.add(offset+x) = true;
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

    let mut first: bool = true;

    // For the first iteration we want to make sure we're keeping the new
    // values separate from the old values, but for all future passes we
    // don't care. If we read a "new" value we'll potentially just remove
    // a roll sooner, which is a good thing.
    let mut read_ptr = memory;
    let write_ptr = unsafe { memory.add(total_size) };

    loop {
        let mut removed = false;

        unsafe {
            rolls.retain(|&index| {
                let neighbours = *read_ptr.add(index - 1) as u8
                    + *read_ptr.add(index + 1) as u8
                    + *read_ptr.add(index - 1 - padded_size) as u8
                    + *read_ptr.add(index + 1 - padded_size) as u8
                    + *read_ptr.add(index - padded_size) as u8
                    + *read_ptr.add(index - 1 + padded_size) as u8
                    + *read_ptr.add(index + 1 + padded_size) as u8
                    + *read_ptr.add(index + padded_size) as u8;

                let remove = neighbours < 4;
                *write_ptr.add(index) = !remove;
                removed |= remove;
                !remove
            })
        }

        if !removed {
            break;
        }

        if first {
            read_ptr = write_ptr;
            let count = roll_count - rolls.len();
            println!("{count}");
            first = false;
        }
    }
    let count = roll_count - rolls.len();
    println!("{count}")
}
