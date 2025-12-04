use std::io::BufRead;
use hashbrown::HashSet;

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut grid = Vec::new();
    let mut hor_length = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line_arr = line.as_bytes();
        hor_length = line_arr.len() + 2;
        grid.push(0);
        for byte in line_arr {
            if *byte == '@' as u8 {
                grid.push(1)
            } else {
                grid.push(0)
            }
        }
        grid.push(0);
    }

    let mut movable_rolls = calculate_neighbors(&mut grid, hor_length);
    let len_initial = movable_rolls.len();

    movable_rolls.iter().for_each(|x| grid[*x] = 0);
    let mut del_counter = 0;
    while movable_rolls.len() > 0 {
        del_counter += 1;
        let roll_idx = *movable_rolls.iter().next().unwrap();
        movable_rolls.remove(&roll_idx);
        if roll_idx > hor_length {
            let top_idx = roll_idx - hor_length - 1;
            let tr = &mut grid[top_idx];
            decrease(tr, top_idx, &mut movable_rolls);
            let tm = &mut grid[top_idx + 1];
            decrease(tm, top_idx + 1, &mut movable_rolls);
            let tl = &mut grid[top_idx + 2];
            decrease(tl, top_idx + 2, &mut movable_rolls);
        }
        let cl = &mut grid[roll_idx - 1];
        decrease(cl, roll_idx - 1, &mut movable_rolls);
        let cr = &mut grid[roll_idx + 1];
        decrease(cr, roll_idx + 1, &mut movable_rolls);
        if roll_idx + hor_length < grid.len() {
            let bot_idx = roll_idx + hor_length - 1;
            let br = &mut grid[bot_idx];
            decrease(br, bot_idx, &mut movable_rolls);
            let bm = &mut grid[bot_idx + 1];
            decrease(bm, bot_idx + 1, &mut movable_rolls);
            let bl = &mut grid[bot_idx + 2];
            decrease(bl, bot_idx + 2, &mut movable_rolls);
        }

    }

    (len_initial, del_counter)
}

fn decrease(val: &mut i32, idx: usize, movable_rolls: &mut HashSet<usize>) {
    if *val > 5 {
        *val -= 1;
    } else if *val > 0 {
        *val = 0;
        movable_rolls.insert(idx);
    }
}

fn calculate_neighbors(grid: &mut Vec<i32>, hor_length: usize) -> HashSet<usize> {
    let mut movable_rolls = HashSet::new();
    // first line
    for hor_idx in 1..(hor_length - 1) {
        if grid[hor_idx] == 1 {
            grid[hor_idx] = 1
                + grid[hor_idx - 1].signum()
                + grid[hor_idx + 1]
                + grid[hor_idx + hor_length - 1]
                + grid[hor_idx + hor_length]
                + grid[hor_idx + hor_length + 1];
            if grid[hor_idx] < 5 {
                movable_rolls.insert(hor_idx);
            }
        }
    }
    // middle part
    let last_line_idx = (grid.len() / hor_length) - 1;
    for ver_idx in 1..last_line_idx {
        let ver_offset_prev = (ver_idx - 1) * hor_length;
        let ver_offset = ver_idx * hor_length;
        let ver_offset_next = (ver_idx + 1) * hor_length;
        for hor_idx in 1..(hor_length - 1) {
            if grid[ver_offset + hor_idx] == 1 {
                grid[ver_offset + hor_idx] = grid[ver_offset_prev + hor_idx - 1].signum()
                    + grid[ver_offset_prev + hor_idx].signum()
                    + grid[ver_offset_prev + hor_idx + 1].signum()
                    + grid[ver_offset + hor_idx - 1].signum()
                    + 1
                    + grid[ver_offset + hor_idx + 1]
                    + grid[ver_offset_next + hor_idx - 1]
                    + grid[ver_offset_next + hor_idx]
                    + grid[ver_offset_next + hor_idx + 1];
                if grid[ver_offset + hor_idx] < 5 {
                    movable_rolls.insert(ver_offset + hor_idx);
                }
            }
        }
    }

    // last line
    let ver_offset_prev = (last_line_idx - 1) * hor_length;
    let ver_offset = last_line_idx * hor_length;
    for hor_idx in 1..(hor_length - 1) {
        if grid[ver_offset + hor_idx] == 1 {
            let prev_line = grid[ver_offset_prev + hor_idx - 1].signum()
                + grid[ver_offset_prev + hor_idx].signum()
                + grid[ver_offset_prev + hor_idx + 1].signum();
            let curr_line =
                1 + grid[ver_offset + hor_idx - 1].signum() + grid[ver_offset + hor_idx + 1];
            grid[ver_offset + hor_idx] = prev_line + curr_line;
            if grid[ver_offset + hor_idx] < 5 {
                movable_rolls.insert(ver_offset + hor_idx);
            }
        }
    }

    movable_rolls
}
