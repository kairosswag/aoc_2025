use std::io::BufRead;
use hashbrown::HashMap;

pub fn run<R>(reader: R) -> (u64, u64)
where
    R: BufRead,
{
    let categories = [vec![], vec![], vec![2], vec![3], vec![2], vec![5], vec![2, 3], vec![7], vec![2], vec![3], vec![2, 5], vec![11], vec![2, 3]];
    let line = reader.lines().next().unwrap().unwrap();
    let split = line.split(",");
    let mut total_sum_p1 = 0;
    let mut total_sum_p2 = 0;
    for line in  split {
        let (first, last) = line.split_once("-").unwrap();
        let (res_p1, res_p2) = sum_doubles(first.parse().unwrap(), first.len(), last.parse().unwrap(), last.len(), &categories);
        total_sum_p1 += res_p1 as u64;
        total_sum_p2 += res_p2 as u64;
    }

    (total_sum_p1, total_sum_p2)
    
}

fn sum_doubles(first_number: u64, first_len: usize, last_number: u64, last_len: usize, category_splits: &[Vec<usize>]) -> (u64, u64) {

    let mut sum_doubles = 0u64;
    let mut sum_total = 0u64;
    let mut already_invalid = HashMap::new();

    for category in first_len..=last_len {
        let category_split = &category_splits[category];

        for split in category_split {
            let len = category / split; // must split fully
            let start_with = 10_u64.pow(len as u32 - 1);
            let end_with = 10_u64.pow(len as u32);
            for i in start_with..end_with {
                let mut adding = 0;
                for _ in 0..*split {
                    adding *= 10_u64.pow(len as u32);
                    adding += i;
                }

                // todo: start and end might be calculeted more strictly
                if adding >= first_number && adding <= last_number && already_invalid.get(&adding).is_none() {
                    already_invalid.insert(adding, *split);
                    sum_total += adding;
                    if *split == 2 {
                        sum_doubles += adding;
                    }
                }
            }
        }
    }



    (sum_doubles, sum_total)
}




