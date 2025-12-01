use std::io::BufRead;

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut current_pos = 50;
    let mut zero_counter_p1 = 0;
    let mut zero_counter_p2 = 0;
    for line in reader.lines() {
        let line_buffer = line.expect("read error");
        let direction = &line_buffer[0..1];
        let value = line_buffer[1..].parse::<i32>().unwrap();
        match direction {
            "L" => {
                let new_pos = current_pos - value;
                if new_pos <= 0 {
                    zero_counter_p2 += (new_pos / 100).abs();
                    if current_pos != 0 {
                        zero_counter_p2 += 1;
                    }
                }
                current_pos = new_pos;
            },
            "R" => {
                if (100 - current_pos) <= value {
                    zero_counter_p2 += 1 + ((value - (100 - current_pos)) / 100);
                }
                current_pos += value
            },
            _ => panic!("unknown direction"),
        }
        current_pos = current_pos.rem_euclid(100);
        if current_pos == 0 {
            zero_counter_p1 += 1;
        }
    }

    (zero_counter_p1, zero_counter_p2 as usize)
}
