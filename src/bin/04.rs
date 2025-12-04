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

        for y in 0..height {
            let y_offset = y * width;

            for x in 0..width {
                if !grid[y_offset + x] {
                    grid_the_second[y_offset + x] = false;
                    continue;
                }

                let mut neighbours = 0;

                if x > 0 {
                    neighbours += grid[y_offset + x - 1] as u8
                }

                if x < width - 1 {
                    neighbours += grid[y_offset + x + 1] as u8
                }

                if y > 0 {
                    neighbours += grid[(y - 1) * width + x] as u8
                }

                if y < height - 1 {
                    neighbours += grid[(y + 1) * width + x] as u8
                }

                if x > 0 && y > 0 {
                    neighbours += grid[(y - 1) * width + x - 1] as u8
                }

                // It was about this point I thought "I really should make a Grid type..."
                if x > 0 && y < height - 1 {
                    neighbours += grid[(y + 1) * width + x - 1] as u8
                }

                if x < width - 1 && y > 0 {
                    neighbours += grid[(y - 1) * width + x + 1] as u8
                }

                if x < width - 1 && y < width - 1 {
                    neighbours += grid[(y + 1) * width + x + 1] as u8
                }

                if neighbours < 4 {
                    removed = true;
                    count += 1;
                    grid_the_second[y_offset + x] = false;
                } else {
                    grid_the_second[y_offset + x] = true;
                }
            }
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
