use std::io;
use day01::run;
pub fn main() {
    let stdin = io::stdin();
    let (p1, p2) = run(stdin.lock());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests_day_01 {
    use crate::run;
    #[test]
    fn test() {
        let test_iput = "L250\nL200";
        assert_eq!((2, 5), run(test_iput.as_bytes()));
    }

    #[test]
    fn test2() {
        let test_iput = "L51\nR100";
        assert_eq!((0, 2), run(test_iput.as_bytes()));
    }
}
