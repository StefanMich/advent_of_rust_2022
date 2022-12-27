use core::panic;
use std::{collections::HashSet};

pub fn main() {
    let line = include_str!("../input.txt");
    
    let part_1 = first_marker(line, 4);
    let part_2 = first_marker(line, 14);
    println!("Part 1: {}, part 2: {}", part_1, part_2);
}

pub fn first_marker(line: &str, packet_size: usize) -> usize {
    let signal = line.chars().collect::<Vec<char>>();
    let packets = signal.windows(packet_size).enumerate();
    for (index, window) in packets{
        if is_marker(window, packet_size){
            return index + packet_size
        }
    }
    panic!("No marker found!")
}

pub fn is_marker(data: &[char], packet_size: usize) -> bool{
    let char_set: HashSet<&char> = HashSet::from_iter(data.iter());
    char_set.len() == packet_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_marker() {
        assert_eq!(first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(first_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_is_start_of_packet_marker(){
        assert_eq!(is_marker(&['a','b','c','d'], 4), true);
        assert_eq!(is_marker(&['a','b','c','a'], 4), false);
    }
}