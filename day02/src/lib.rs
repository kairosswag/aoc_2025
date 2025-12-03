use std::io::BufRead;
use hashbrown::HashMap;
use rayon::prelude::*;

pub fn run<R>(reader: R) -> (u64, u64)
where
    R: BufRead,
{
    let categories = [vec![], vec![], vec![2], vec![3], vec![2], vec![5], vec![2, 3], vec![7], vec![2], vec![3], vec![2, 5], vec![11], vec![2, 3]];
    let line = reader.lines().next().unwrap().unwrap();
    let split = line.split(",").enumerate();
    split.par_bridge().map(|(_, line)| line.split_once("-").unwrap() )
        .map(|(first, last)| sum_doubles(first.parse().unwrap(), first.len(), last.parse().unwrap(), last.len(), &categories))
        .reduce(|| (0, 0), |acc, (a, b)| (acc.0 + a, acc.1 + b))

}

fn sum_doubles(first_number: u64, first_len: usize, last_number: u64, last_len: usize, category_splits: &[Vec<usize>]) -> (u64, u64) {

    let mut sum_doubles = 0u64;
    let mut sum_total = 0u64;
    let mut already_invalid = HashMap::new();

    for category in first_len..=last_len {
        let category_split = &category_splits[category];

        'split: for split in category_split {
            let len = category / split; // must split fully
            let start_with = calculate_start(category, len, first_len, first_number);
            let end_with = 10_u64.pow(len as u32);
            for i in start_with..end_with {
                let mut adding = 0;
                for _ in 0..*split {
                    adding *= 10_u64.pow(len as u32);
                    adding += i;
                }
                if adding > last_number {
                    continue 'split;
                }
                // todo: start and end might be calculeted more strictly
                if already_invalid.get(&adding).is_none() {

                    already_invalid.insert(adding, *split);
                    sum_total += adding;
                    if *split == 2 {
                        // println!("Number: {first_number}-{last_number} -> {adding}");
                        sum_doubles += adding;
                    }
                }
            }
        }
    }



    (sum_doubles, sum_total)
}

pub fn calculate_start(category: usize, len_split: usize, first_len: usize, first_number: u64) -> u64 {
    if category == first_len {
        let mut start_at = true;
        let (particle, mut curr_remainder) = cut_back((first_len - len_split) as u32, first_number);
        // println!("stepping to {}", first_len/len_split);
        for step in 2..=(first_len/len_split) {
            let (next_particle, rem) = cut_back((first_len - len_split * step) as u32, curr_remainder);
            if particle > next_particle {
                start_at = true;
                break;
            } else if particle < next_particle {
                start_at = false;
                break;
            } else {
                curr_remainder = rem;
            }
        }
        if start_at {
            particle
        } else {
            particle + 1
        }
    } else {
        10_u64.pow(len_split as u32 - 1)
    }
}

fn cut_back(num_back: u32, number: u64) -> (u64, u64) {
    let particle = number / 10_u64.pow(num_back);
    let remainder = number - particle * 10_u64.pow(num_back);
    // println!("Cutting back {number} to {num_back}: {particle} {remainder}");
    (particle, remainder)
}




