use std::io::BufRead;

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut hor_neighbors = Vec::new();
    let mut grid = Vec::new();
    let mut hor_length = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        hor_length = line.len() + 2;
        let line_arr = line.as_bytes();
        hor_neighbors.push(0);
        grid.push(0);
        let mut prev_val = 0;
        let mut next_val = if line_arr[0] == '@' as u8 {
            1
        } else {
            0
        };
        for i in 0..(line_arr.len()-1) {
            let curr_val = next_val;
            next_val = if line_arr[i+1] == '@' as u8 {
                1
            } else {
                0
            };
            if curr_val == 1 {
                grid.push(1);
                hor_neighbors.push(prev_val + 1 + next_val);
            } else {
                grid.push(0);
                hor_neighbors.push(prev_val + next_val);
            }
            prev_val = curr_val;
        }
        if next_val == 1 {
            grid.push(1);
            hor_neighbors.push(prev_val + 1);
        } else {
            grid.push(0);
            hor_neighbors.push(prev_val);
        }
        hor_neighbors.push(0);
        grid.push(0);
    }

    let (mut up_roll_grid, mut up_hor_neighbors, mut rolls_removed) = reduce_rolls(grid, hor_neighbors, hor_length);
    let movable_rolls = rolls_removed;
    let mut rolls_removed_total = rolls_removed;
    while rolls_removed > 0 {
        (up_roll_grid, up_hor_neighbors, rolls_removed) = reduce_rolls(up_roll_grid, up_hor_neighbors, hor_length);
        rolls_removed_total += rolls_removed;
    }



    (movable_rolls, rolls_removed_total)
    
}

fn reduce_rolls(roll_grid: Vec<i32>, hor_neighbors: Vec<i32>, hor_length: usize) -> (Vec<i32>, Vec<i32>, usize) {
    let mut rolls_removed = 0;
    let mut updated_roll_grid = roll_grid.clone();
    let mut updated_hor_neighbors = hor_neighbors.clone();


    for hor_idx in 1..(hor_length-1) {
        if roll_grid[0*hor_length + hor_idx] == 1 {
            let neighbors = hor_neighbors[0 * hor_length + hor_idx] + hor_neighbors[1 * hor_length + hor_idx] - 1;
            if neighbors < 4 {
                rolls_removed += 1;
                remove_roll_at(&mut updated_roll_grid, &mut updated_hor_neighbors, hor_idx, 0, hor_length);
            }
        }
    }


    for ver_idx in 1..((roll_grid.len() /hor_length )- 1) {
        let curr_start_idx = ver_idx * hor_length;
        let prev_start_idx = (ver_idx - 1) * hor_length;
        let next_start_idx = (ver_idx+1) * hor_length;

        for hor_idx in 1..(hor_length-1) {
            if roll_grid[ver_idx * hor_length + hor_idx] == 1 {
                let neighbors = hor_neighbors[prev_start_idx + hor_idx] + hor_neighbors[next_start_idx + hor_idx] + hor_neighbors[curr_start_idx + hor_idx] - 1;
                if neighbors < 4 {
                    rolls_removed += 1;
                    remove_roll_at(&mut updated_roll_grid, &mut updated_hor_neighbors, hor_idx, ver_idx, hor_length);
                }
            }
        }
    }

    let last_line_veridx = (roll_grid.len() / hor_length)-1;
    let ll_start_idx = last_line_veridx * hor_length;
    let prev_start_idx = (last_line_veridx - 1) * hor_length;
    for hor_idx in 1..(hor_length-1) {
        if roll_grid[last_line_veridx * hor_length + hor_idx] == 1 {
            let neighbors = hor_neighbors[prev_start_idx + hor_idx] + hor_neighbors[ll_start_idx + hor_idx] - 1;
            if neighbors < 4 {
                rolls_removed += 1;
                remove_roll_at(&mut updated_roll_grid, &mut updated_hor_neighbors, hor_idx, last_line_veridx, hor_length);
            }
        }
    }



    (updated_roll_grid, updated_hor_neighbors, rolls_removed)

}

fn remove_roll_at(updated_roll_grid: &mut Vec<i32>, updated_hor_neighbors: &mut Vec<i32>, hor_idx: usize, vert_idx: usize, hor_length: usize) {
    updated_roll_grid[vert_idx * hor_length + hor_idx] = 0;
    updated_hor_neighbors[vert_idx * hor_length + hor_idx] -= 1;
    let left_nb = &mut updated_hor_neighbors[vert_idx * hor_length + hor_idx - 1];
    *left_nb = if *left_nb == 0 {
        0
    } else {
        *left_nb - 1
    };
    let right_nb = &mut updated_hor_neighbors[vert_idx * hor_length + hor_idx + 1];
    *right_nb = if *right_nb == 0 {
        0
    } else {
        *right_nb - 1
    };
}



