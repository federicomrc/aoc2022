use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read},
};

fn main() -> Result<(), std::io::Error> {
    part_one()?;

    part_two()?;

    Ok(())
}

fn part_one() -> Result<(), std::io::Error> {
    println!("Part one");

    let mut buf: String = String::new();
    let _ = get_input_handle()?.read_to_string(&mut buf)?;

    if let Some(idx) = get_first_different_sequence_index(&buf, 4) {
        println!("Starting at: {}", idx);
    }

    Ok(())
}

fn part_two() -> Result<(), std::io::Error> {
    println!("Part two");

    let mut buf: String = String::new();
    let _ = get_input_handle()?.read_to_string(&mut buf)?;

    if let Some(idx) = get_first_different_sequence_index(&buf, 14) {
        println!("Starting at: {}", idx);
    }

    Ok(())
}

fn get_input_handle() -> Result<BufReader<File>, std::io::Error> {
    let f = File::open("./input.txt")?;

    Ok(BufReader::new(f))
}

fn get_first_different_sequence_index(signal: &str, len: usize) -> Option<usize> {
    if signal.len() < len - 1 {
        return None;
    }

    for idx in len..signal.len() {
        let sl = &signal[idx - len..idx];

        let mut uniq = HashSet::new();
        let uniq = sl.chars().all(|x| uniq.insert(x));

        if uniq {
            return Some(idx);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::get_first_different_sequence_index;

    #[test]
    fn part_one_test_1() {
        let s = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let r = get_first_different_sequence_index(s, 4);

        assert_eq!(r, Some(5));
    }

    #[test]
    fn part_one_test_2() {
        let s = "nppdvjthqldpwncqszvftbrmjlhg";
        let r = get_first_different_sequence_index(s, 4);

        assert_eq!(r, Some(6));
    }

    #[test]
    fn part_one_test_3() {
        let s = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let r = get_first_different_sequence_index(s, 4);

        assert_eq!(r, Some(10));
    }

    #[test]
    fn part_one_test_4() {
        let s = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let r = get_first_different_sequence_index(s, 4);

        assert_eq!(r, Some(11));
    }

    #[test]
    fn part_two_test_1() {
        let s = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let r = get_first_different_sequence_index(s, 14);

        assert_eq!(r, Some(19));
    }

    #[test]
    fn part_two_test_2() {
        let s = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let r = get_first_different_sequence_index(s, 14);

        assert_eq!(r, Some(23));
    }

    #[test]
    fn part_two_test_3() {
        let s = "nppdvjthqldpwncqszvftbrmjlhg";
        let r = get_first_different_sequence_index(s, 14);

        assert_eq!(r, Some(23));
    }

    #[test]
    fn part_two_test_4() {
        let s = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let r = get_first_different_sequence_index(s, 14);

        assert_eq!(r, Some(29));
    }

    #[test]
    fn part_two_test_5() {
        let s = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let r = get_first_different_sequence_index(s, 14);

        assert_eq!(r, Some(26));
    }
}
