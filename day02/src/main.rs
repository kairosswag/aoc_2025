use day02::run;
use rayon::ThreadPoolBuilder;
use std::io;
use std::io::Write;

pub fn main() {
    let stdin = io::stdin();
    ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    let mut stdout = io::stdout();
    stdout.lock().write(&['g' as u8, 'o' as u8, '\n' as u8]).expect("Could not write out.");
    stdout.flush().expect("Could not write 'go'");
    let (p1, p2) = run(stdin.lock());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests_day_02 {
    use crate::run;


    #[test]
    fn test() {
        let test_iput = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!((1227775554, 4174379265), run(test_iput.as_bytes()));
    }

    #[test]
    fn test_single() {
        let test_iput = "11-22";
        assert_eq!((33, 33), run(test_iput.as_bytes()));
    }

    #[test]
    fn test_single_2() {
        let test_iput = "95-115";
        assert_eq!((99, 210), run(test_iput.as_bytes()));
    }

    #[test]
    fn test_single_3() {
        let test_iput = "565653-565659";
        assert_eq!((0, 565656), run(test_iput.as_bytes()));
    }

    #[test]
    fn test_cutting() {
        assert_eq!(13, day02::calculate_start(6, 2, 6, 121312)); // 12
    }


    #[test]
    fn test_cutting_2() {
        assert_eq!(566, day02::calculate_start(6, 3, 6, 565653)); // 12
    }


}
