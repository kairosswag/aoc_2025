use std::io::BufRead;


pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut sum_p1 = 0;
    let mut sum_p2 = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let line = line.as_bytes();
        let (idx_start, upper) = calc_highest(&line[0..line.len() - 1]);
        let (_, lower) = calc_highest(&line[idx_start + 1..]);

        sum_p1 += ((upper - '0' as u8) * 10  + (lower - '0' as u8)) as usize;

        let mut curr_start_idx = 0;
        let mut inter = 0usize;
        for i in (0..12).rev() {
            let val = calc_highest(&line[curr_start_idx..line.len() - i]);
            inter *= 10;
            inter += (val.1 - '0' as u8) as usize;
            curr_start_idx += val.0 + 1;
        }
        sum_p2 += inter;
    }

    (sum_p1, sum_p2)
    
}

fn calc_highest(char_array: &[u8]) -> (usize, u8) {
    let ( mut pos, mut max) = (0, 0);
    for i in 0..char_array.len() {
        let val = char_array[i];
        if val > max {
            (pos, max) = (i, val);
            if max == '9' as u8 {
                break;
            }
        }
    }
    (pos, max)
}



