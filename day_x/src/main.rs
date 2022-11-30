pub fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap());
    println!("{:#?}", lines);

}