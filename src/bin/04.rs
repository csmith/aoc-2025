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

    let mut source_y_offset = 0;
    let mut dest_y_offset = width+2;
    for _ in 0..height {
        for x in 0..width {
            grid[dest_y_offset+x+1] = raw_grid[source_y_offset+x]
        }
        source_y_offset += width;
        dest_y_offset += width + 2;
    }

    let mut first: bool = true;
    let mut count: u32 = 0;

    loop {
        let mut removed = false;
        let mut y_offset = width+2;
        let mut last_y_offset = 0;
        let mut next_y_offset = (width+2)*2;

        let grid_ptr = grid.as_ptr();
        let grid_the_second_ptr = grid_the_second.as_mut_ptr();
        
        for _ in 1..=height {
            let mut index = y_offset;
            for x in 1..=width {
                index += 1;
                unsafe {
                    if !*grid_ptr.add(index) {
                        *grid_the_second_ptr.add(index) = false;
                        continue;
                    }

                    let neighbours = *grid_ptr.add(index - 1) as u8
                            + *grid_ptr.add(index + 1) as u8
                            + *grid_ptr.add(last_y_offset + x) as u8
                            + *grid_ptr.add(next_y_offset + x) as u8
                            + *grid_ptr.add(last_y_offset + x - 1) as u8
                            + *grid_ptr.add(next_y_offset + x - 1) as u8
                            + *grid_ptr.add(last_y_offset + x + 1) as u8
                            + *grid_ptr.add(next_y_offset + x + 1) as u8;

                    let remove = neighbours < 4;
                    *grid_the_second_ptr.add(index) = !remove;
                    removed |= remove;
                    count += remove as u32;
                }
            }

            last_y_offset = y_offset;
            y_offset = next_y_offset;
            next_y_offset += width+2;
        }

        if !removed {
            break;
        }

        if first {
            println!("{count}");
            first = false;
        }
        
        std::mem::swap(&mut grid, &mut grid_the_second);
    }
    println!("{count}")
}
