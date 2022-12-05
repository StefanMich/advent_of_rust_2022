use std::collections::btree_set::BTreeSet;
pub fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let duplicate = find_duplicate_in_rucksack(l);
            let score = char_to_score(duplicate);
            println!("{} {}", duplicate, score);
            score
        }
    ).collect::<Vec<usize>>();
    println!("{:#?}", lines.iter().sum::<usize>());
}

pub fn char_to_score(duplicate: char) -> usize {
    let ascii_value = duplicate as usize;
    if (97..122+1).contains(&ascii_value) {
        ascii_value - 96
    }
    else if (65..90+1).contains(&ascii_value)  {
        ascii_value - 38
    }
    else {
        panic!("Unknown char")
    }
}

pub fn find_duplicate_in_rucksack(rucksack_contents: &str) -> char {
    let compartments = find_compartment_contents(rucksack_contents);
    find_duplicate_item(compartments)
}

pub fn find_compartment_contents(rucksack_contents: &str) -> (&str, &str) {
    let length = rucksack_contents.len();
    rucksack_contents.split_at(length / 2)
}


pub fn find_duplicate_item(rucksack_contents: (&str, &str)) -> char {
    let left_compartment = unique_items(rucksack_contents.0);
    let right_compartment = unique_items(rucksack_contents.1);
    let c = left_compartment.intersection(&right_compartment);
    
    let duplicate = c.cloned().collect::<Vec<char>>();
    if duplicate.len() != 1 {
        panic!("More than one duplicate")
    };
    duplicate[0]
}

pub fn unique_items(compartment_contents: &str) -> BTreeSet<char>{
    let mut count = BTreeSet::new();

    for c in compartment_contents.chars() {
        count.insert(c);
    }
    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points_per_line() {
        assert_eq!(find_duplicate_in_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
    }

    #[test]
    fn test_char_to_score() {
        assert_eq!(char_to_score('L'), 38);
        assert_eq!(char_to_score('t'), 20);
    }
}