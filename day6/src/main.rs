use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file not found");

    let start_of_packet = find_start_of_packet(input.as_str()).unwrap();
    println!("The first start-of-packet marker is at {start_of_packet}");

    let start_of_packet = find_start_of_message(input.as_str()).unwrap();
    println!("The first start-of-message marker is at {start_of_packet}");
}

fn find_start_of_packet(signal: &str) -> Result<usize, &'static str> {
    find_start_of_distinct_slice(signal, 4)
}

fn find_start_of_message(signal: &str) -> Result<usize, &'static str> {
    find_start_of_distinct_slice(signal, 14)
}

fn find_start_of_distinct_slice(signal: &str, slice_len: usize) -> Result<usize, &'static str> {
    let end = signal.len() - slice_len;
    for i in 0..end {
        if !contains_duplicate(&signal[i..i + slice_len]) {
            return Ok(i + slice_len);
        }
    }

    Err("Start of slice not found")
}

fn contains_duplicate(signal: &str) -> bool {
    let mut chars = String::new();
    for c in signal.chars() {
        if chars.contains(c) {
            return true;
        }
        chars.push(c);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate_true() {
        let signal = "bvwb";
        assert!(contains_duplicate(signal))
    }

    #[test]
    fn test_contains_duplicate_false() {
        let signal = "bvwa";
        assert!(!contains_duplicate(signal))
    }

    #[test]
    fn test_find_start_of_packet_1() {
        let signal = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(7, find_start_of_packet(signal).unwrap());
    }

    #[test]
    fn test_find_start_of_packet_2() {
        let signal = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(5, find_start_of_packet(signal).unwrap());
    }

    #[test]
    fn test_find_start_of_packet_3() {
        let signal = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(6, find_start_of_packet(signal).unwrap());
    }

    #[test]
    fn test_find_start_of_packet_4() {
        let signal = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(10, find_start_of_packet(signal).unwrap());
    }

    #[test]
    fn test_find_start_of_packet_5() {
        let signal = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(11, find_start_of_packet(signal).unwrap());
    }

    #[test]
    fn test_find_start_of_message_1() {
        let signal = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(19, find_start_of_message(signal).unwrap());
    }

    #[test]
    fn test_find_start_of_message_2() {
        let signal = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(23, find_start_of_message(signal).unwrap());
    }

    #[test]
    fn test_find_start_of_message_3() {
        let signal = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(23, find_start_of_message(signal).unwrap());
    }

    #[test]
    fn test_find_start_of_message_4() {
        let signal = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(29, find_start_of_message(signal).unwrap());
    }

    #[test]
    fn test_find_start_of_message_5() {
        let signal = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(26, find_start_of_message(signal).unwrap());
    }
}
