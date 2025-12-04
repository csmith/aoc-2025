fn main() {
    let file = std::fs::read_to_string("inputs/04.txt").expect("Couldn't read file");
    let bytes = file.as_bytes();
    let width = bytes
        .iter()
        .position(|&b| b == b'\n' || b == b'\r')
        .expect("No new lines?");

    let mut grid: Vec<bool> = bytes
        .iter()
        .filter_map(|&b| match b {
            b'@' => Some(true),
            b'.' => Some(false),
            _ => None,
        })
        .collect();

    let mut grid_the_second: Vec<bool> = vec![false; grid.len()];
    let height = grid.len() / width;

    let mut first: bool = true;
    let mut count: u32 = 0;

    loop {
        let mut removed = false;
        let mut y_offset = 0;
        let mut last_y_offset = 0;
        let mut next_y_offset = width;

        for y in 0..height {
            for x in 0..width {
                if !grid[y_offset + x] {
                    grid_the_second[y_offset + x] = false;
                    continue;
                }

                let mut neighbours = 0;

                neighbours += (x > 0 && grid[y_offset + x - 1]) as u8;
                neighbours += (x < width - 1 && grid[y_offset + x + 1]) as u8;
                neighbours += (y > 0 && grid[last_y_offset + x]) as u8;
                neighbours += (y < height - 1 && grid[next_y_offset + x]) as u8;
                neighbours += (x > 0 && y > 0 && grid[last_y_offset + x - 1]) as u8;
                neighbours += (x > 0 && y < height - 1 && grid[next_y_offset + x - 1]) as u8;
                neighbours += (x < width - 1 && y > 0 &&  grid[last_y_offset + x + 1]) as u8;
                neighbours += (x < width - 1 && y < height - 1 && grid[next_y_offset + x + 1]) as u8;

                let remove = neighbours < 4;
                grid_the_second[y_offset + x] = !remove;
                if remove {
                    removed = true;
                    count += 1;
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
