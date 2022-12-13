// [P]     [L]         [T]            
// [L]     [M] [G]     [G]     [S]    
// [M]     [Q] [W]     [H] [R] [G]    
// [N]     [F] [M]     [D] [V] [R] [N]
// [W]     [G] [Q] [P] [J] [F] [M] [C]
// [V] [H] [B] [F] [H] [M] [B] [H] [B]
// [B] [Q] [D] [T] [T] [B] [N] [L] [D]
// [H] [M] [N] [Z] [M] [C] [M] [P] [P]
//  1   2   3   4   5   6   7   8   9 

use regex::Regex;

pub fn main() {
    let init = vec![
        vec!['H','B','V','W','N','M','L','P',],
        vec!['M','Q','H',],
        vec!['N','D','B','G','F','Q','M','L',],
        vec!['Z','T','F','Q','M','W','G',],
        vec!['M','T','H','P',],
        vec!['C','B','M','J','D','H','G','T',],
        vec!['M','N','B','F','V','R',],
        vec!['P','L','H','M','R','G','S',],
        vec!['P','D','B','C','N',],
    ];

    let mut result = init.clone();

    let lines = include_str!("../input.txt")
        .lines()
        .map(|l| {
            println!("{}", l);
            let re = Regex::new(r"^\D*(\d*)\D*(\d*)\D*(\d*)$").unwrap();
            let captures = re.captures(l).unwrap();
            print!("{:#?}", captures);
            let amount = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let to = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
            (amount, from, to)
        });

    for (amount, from, to) in lines {
        result = move_crates(result, amount, from, to);
    }
    for stack in result {
        print!("{}", stack.last().unwrap());
    }
    println!("")

}

pub fn move_crates(state: Vec<Vec<char>>, amount: usize, from: usize, to:usize) -> Vec<Vec<char>>{
    let mut result = state.clone();
    for _  in 0..amount {
        let from_crate = result[from - 1].pop().unwrap();
        result[to - 1].push(from_crate);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_1() {
        let init: Vec<Vec<char>> = vec![vec!['A', 'B'], vec!['C']];
        let result = move_crates(init, 1, 1, 2);
        assert_eq!(result[0], vec!['A']);
        assert_eq!(result[1], vec!['C', 'B']);
    }

    #[test]
    fn move_2() {
        let init: Vec<Vec<char>> = vec![vec!['A', 'B'], vec!['C']];
        let result = move_crates(init, 2, 1, 2);
        assert_eq!(result[0], Vec::<char>::new());
        assert_eq!(result[1], vec!['C', 'B', 'A']);
    }

    #[test]
    fn te(){
        let s = "asd 3";
        let re = Regex::new(r"^\D*(\d*)").unwrap();
        let captures = re.captures(s);
        println!("{:#?}", captures);

    }
}