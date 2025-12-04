use memmap2::Mmap;
use std::fs::File;

fn main() {
    let file = File::open("inputs/04.txt").expect("Couldn't open file");
    let mut grid: Vec<bool> = unsafe { Mmap::map(&file) }
        .expect("Couldn't read file")
        .iter()
        .filter_map(|&b| match b {
            b'@' => Some(true),
            b'.' => Some(false),
            _ => None,
        })
        .collect();

    let mut grid_the_second: Vec<bool> = vec![false; grid.len()];
    // Assuming all grids are square...
    let width = grid.len().isqrt();
    let height = width;

    let mut first: bool = true;
    let mut count: u32 = 0;

    loop {
        let mut removed = false;
        let mut y_offset = 0;
        let mut last_y_offset = 0;
        let mut next_y_offset = width;

        for y in 0..height {
            let up = y > 0;
            let down = y < height - 1;

            for x in 0..width {
                unsafe {
                    if !grid.get_unchecked(y_offset + x) {
                        *grid_the_second.get_unchecked_mut(y_offset + x) = false;
                        continue;
                    }

                    let left = x > 0;
                    let right = x < width - 1;
                    let mut neighbours = 0;

                    if up && down && left && right {
                        // Most common case, don't bother with the 8 individual branches
                        neighbours += *grid.get_unchecked(y_offset + x - 1) as u8
                            + *grid.get_unchecked(y_offset + x + 1) as u8
                            + *grid.get_unchecked(last_y_offset + x) as u8
                            + *grid.get_unchecked(next_y_offset + x) as u8
                            + *grid.get_unchecked(last_y_offset + x - 1) as u8
                            + *grid.get_unchecked(next_y_offset + x - 1) as u8
                            + *grid.get_unchecked(last_y_offset + x + 1) as u8
                            + *grid.get_unchecked(next_y_offset + x + 1) as u8;
                    } else {
                        neighbours += (left && *grid.get_unchecked(y_offset + x - 1)) as u8;
                        neighbours += (right && *grid.get_unchecked(y_offset + x + 1)) as u8;
                        neighbours += (up && *grid.get_unchecked(last_y_offset + x)) as u8;
                        neighbours += (down && *grid.get_unchecked(next_y_offset + x)) as u8;
                        neighbours += (left && up && *grid.get_unchecked(last_y_offset + x - 1)) as u8;
                        neighbours += (left && down && *grid.get_unchecked(next_y_offset + x - 1)) as u8;
                        neighbours += (right && up && *grid.get_unchecked(last_y_offset + x + 1)) as u8;
                        neighbours += (right && down && *grid.get_unchecked(next_y_offset + x + 1)) as u8;
                    }

                    let remove = neighbours < 4;
                    *grid_the_second.get_unchecked_mut(y_offset + x) = !remove;
                    if remove {
                        removed = true;
                        count += 1;
                    }
                }
            }

            last_y_offset = y_offset;
            y_offset = next_y_offset;
            next_y_offset += width;
        }

        if !removed {
            break;
        }

        if first {
            println!("{count}");
        }

        first = false;
        std::mem::swap(&mut grid, &mut grid_the_second);
    }
    println!("{count}")
}
