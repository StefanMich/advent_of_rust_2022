const OPPONENT_ROCK: char = 'A';
const OPPONENT_PAPER: char = 'B';
const OPPONENT_SCISSORS: char = 'C';

const YOU_ROCK: char = 'X';
const YOU_PAPER: char = 'Y';
const YOU_SCISSORS: char = 'Z';

pub fn main() {
    let lines: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let (opponent, choice) = l.split_once(" ").unwrap();
            let opponent = opponent.chars().next().unwrap();
            let choice = choice.chars().next().unwrap();
            calculate_score(opponent, choice)
        }).collect();
    //println!("{:#?}", lines);
    println!("{}",lines.iter().sum::<usize>())
}

pub fn calculate_score(opponent: char, choice: char) -> usize{
    points_by_outcome(opponent, choice) + points_by_choice(choice)
}

pub fn points_by_choice(choice: char) -> usize{
    match choice {
        YOU_ROCK => 1,
        YOU_PAPER => 2,
        YOU_SCISSORS => 3,
        _ => panic!("Unknown choice")
    }
}

pub fn points_by_outcome(opponent: char, choice: char) -> usize{
    let combination = &(opponent, choice);
    let draw_combinations = vec!(
        (OPPONENT_ROCK, YOU_ROCK),
        (OPPONENT_PAPER, YOU_PAPER),
        (OPPONENT_SCISSORS, YOU_SCISSORS)
    );
    if draw_combinations.contains(combination) {return 3}
    
    let win_combinations = vec!(
        (OPPONENT_ROCK, YOU_PAPER),
        (OPPONENT_PAPER, YOU_SCISSORS),
        (OPPONENT_SCISSORS, YOU_ROCK));
    if win_combinations.contains(combination){return 6}

    return 0
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