use std::io;
use day05::run;
pub fn main() {
    let stdin = io::stdin();
    let (p1, p2) = run(stdin.lock());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests_day_05 {
    use crate::run;
    #[test]
    fn test() {
        let test_iput = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!((3, 14), run(test_iput.as_bytes()));
    }
}
