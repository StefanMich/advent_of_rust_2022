pub fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let (first_sections, second_sections) = line_to_sections(l);
            fully_contains(first_sections, second_sections)
});
    println!("{:#?}", lines.clone().collect::<Vec<bool>>());
    println!("{}", lines.collect::<Vec<bool>>().iter().filter(|&x|x == &true).count())
}

pub fn line_to_sections(line: &str) -> ((usize, usize), (usize, usize)) {
    let (first, second) = line.split_once(",").unwrap();
    (sections_as_tuple(first), sections_as_tuple(second))
    

}

pub fn sections_as_tuple(sections: &str) -> (usize, usize){
    let sections = sections.split_once("-").unwrap();
    (sections.0.parse::<usize>().unwrap(), sections.1.parse::<usize>().unwrap())
}

pub fn fully_contains(first_sections: (usize, usize), second_sections: (usize, usize)) -> bool {
    (first_sections.0 <= second_sections.0 && first_sections.1 >= second_sections.1) ||
    (second_sections.0 <= first_sections.0 && second_sections.1 >= first_sections.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fully_contains() {
        assert_eq!(fully_contains((2,8),(3,7)), true);
        assert_eq!(fully_contains((4,6),(6,6)), true);
        assert_eq!(fully_contains((2,3),(1,3)), true);
        assert_eq!(fully_contains((5,7),(7,9)), false);
        assert_eq!(fully_contains((2,4),(6,8)), false);
    }
}