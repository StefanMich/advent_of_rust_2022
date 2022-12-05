pub fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap());
    println!("{:#?}", lines);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points_per_line() {
        assert_eq!(calculate_score('A', 'Y'), 8);
        assert_eq!(calculate_score('B', 'X'), 1);
        assert_eq!(calculate_score('C', 'Z'), 6);
    }
}