use std::io;
use day03::run;
pub fn main() {
    let stdin = io::stdin();
    let (p1, p2) = run(stdin.lock());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests_day_03 {
    use crate::run;
    #[test]
    fn test() {
        let test_iput = "987654321111111";
        assert_eq!((98, 987654321111), run(test_iput.as_bytes()));
    }

    #[test]
    fn test_2() {
        let test_iput = "811111111111119";
        assert_eq!((89, 811111111119), run(test_iput.as_bytes()));
    }

    #[test]
    fn test_all() {
        let test_iput = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!((357, 3121910778619), run(test_iput.as_bytes()));
    }

}
