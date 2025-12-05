use memmap2::Mmap;
use std::fs::File;

fn main() {
    let file = File::open("inputs/04.txt").expect("Couldn't open file");
    let raw_grid: Vec<bool> = unsafe { Mmap::map(&file) }
        .expect("Couldn't read file")
        .iter()
        .filter_map(|&b| match b {
            b'@' => Some(true),
            b'.' => Some(false),
            _ => None,
        })
        .collect();

    // Assuming all grids are square...
    let width = raw_grid.len().isqrt();
    let height = width;

    let mut grid: Vec<bool> = vec![false; (width+2)*(height+2)];
    let mut grid_the_second: Vec<bool> = vec![false; (width+2)*(height+2)];
    let mut rolls: Vec<(usize, usize)> = Vec::with_capacity(15000);

    let mut source_y_offset = 0;
    let mut dest_y_offset = width+2;
    for y in 0..height {
        for x in 0..width {
            let val = raw_grid[source_y_offset+x];
            grid[dest_y_offset+x+1] = val;
            if val {
                rolls.push((y, x))
            }
        }
        source_y_offset += width;
        dest_y_offset += width + 2;
    }

    let mut first: bool = true;
    let mut count: u32 = 0;

    // For the first iteration we want to make sure we're keeping the new
    // values separate from the old values, but for all future passes we
    // don't care. If we read a "new" value we'll potentially just remove
    // a roll sooner, which is a good thing.
    let mut read_ptr = grid.as_ptr();
    let write_ptr = grid_the_second.as_mut_ptr();

    loop {
        let mut removed = false;

        unsafe {
            rolls.retain(|(y, x)| {
                let index = (y + 1) * (width + 2) + x + 1;

                let neighbours = *read_ptr.add(index - 1) as u8
                    + *read_ptr.add(index + 1) as u8
                    + *read_ptr.add(index - 1 - width - 2) as u8
                    + *read_ptr.add(index + 1 - width - 2) as u8
                    + *read_ptr.add(index - width - 2) as u8
                    + *read_ptr.add(index - 1 + width + 2) as u8
                    + *read_ptr.add(index + 1 + width + 2) as u8
                    + *read_ptr.add(index + width + 2) as u8;

                let remove = neighbours < 4;
                *write_ptr.add(index) = !remove;
                removed |= remove;
                count += remove as u32;
                !remove
            })
        }

        if !removed {
            break;
        }

        if first {
            read_ptr = grid_the_second.as_ptr();
            println!("{count}");
            first = false;
        }
    }
    println!("{count}")
}
