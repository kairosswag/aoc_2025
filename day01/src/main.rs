use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use day01::run;
pub fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    stdout.lock().write(&['g' as u8, 'o' as u8, '\n' as u8]).expect("Could not write out.");
    stdout.flush().expect("Could not write 'go'");
    let (p1, p2) = run(stdin.lock());

    let mut lock = stdout.lock();
    lock.write(&p1.to_string().as_bytes()).expect("Could not write out.");
    lock.write(&['\n' as u8]).expect("Could not write out.");
    lock.write(&p2.to_string().as_bytes()).expect("Could not write out.");
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
