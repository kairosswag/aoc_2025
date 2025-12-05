use std::collections::BTreeMap;
use std::io::BufRead;
use hashbrown::HashSet;

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut from_map: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    let mut to_map: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    let mut end_idx = Vec::new();
    let mut line_iter = reader.lines().into_iter();
    line_iter.by_ref().map(|v| v.unwrap()).enumerate().take_while(|(_, l)| l.is_empty() == false).for_each(|(idx, line)| {
        let splits = line.split_once('-').unwrap();

        let lower: usize = splits.0.parse().unwrap();
        let mut ins_dupl_idx = 0;
        while from_map.contains_key(&(lower, ins_dupl_idx)) {
            ins_dupl_idx += 1;
        }
        from_map.insert((lower, ins_dupl_idx), idx);

        let upper: usize = splits.1.parse().unwrap();
        ins_dupl_idx = 0;
        while to_map.contains_key(&(upper, ins_dupl_idx)) {
            ins_dupl_idx += 1;
        }
        to_map.insert((upper, ins_dupl_idx), idx);
        end_idx.push(upper);
    });

    let mut fresh_count = 0;
    'items: for item in line_iter {
        let item: usize = item.unwrap().parse().unwrap();
        let res = from_map.range(..=(item, usize::MAX)).map(|val| *val.1).collect::<HashSet<usize>>();
        for (_, xv) in to_map.range((item, 0)..) {
            if res.contains(xv) {
                fresh_count += 1;
                continue 'items;
            }
        }
    }

    let first = from_map.iter().next().unwrap();
    let mut curr_start = first.0.0;
    let mut curr_max_end = end_idx[*first.1];
    let mut total_fresh = 0;
    for ((lower, _), idx) in from_map.iter() {
        if *lower > curr_max_end {
            let add = (curr_max_end - curr_start) + 1;
            total_fresh += add;
            curr_start = *lower;
            curr_max_end = end_idx[*idx];
        } else {
            let range_end = end_idx[*idx];
            if range_end > curr_max_end {
                curr_max_end = range_end;
            }
        }
    }
    let add = (curr_max_end - curr_start) + 1;
    total_fresh += add;

    (fresh_count, total_fresh)
    
}



