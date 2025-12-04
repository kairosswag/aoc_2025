use std::io;
use std::io::Write;
use day04::run;
pub fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    stdout.lock().write(&['g' as u8, 'o' as u8, '\n' as u8]).expect("Could not write out.");
    stdout.flush().expect("Could not write 'go'");
    let (p1, p2) = run(stdin.lock());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests_day_04 {
    use crate::run;
    #[test]
    fn test() {
        let test_iput = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!((13, 43), run(test_iput.as_bytes()));
    }

}
