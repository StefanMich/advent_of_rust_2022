pub fn main() {
    let lines = include_str!("../input.txt")
        .lines();

    let mut inventories: Vec<Vec<u32>> = Vec::new();
    let mut current_elf_inventory: Vec<u32> = Vec::new();
    for line in lines{
        match line {
            "" => {
                inventories.push(current_elf_inventory);
                current_elf_inventory = Vec::new();
            }
            _ => current_elf_inventory.push(line.parse::<u32>().unwrap())
        }
    }
    inventories.push(current_elf_inventory);

    let inventory_totals = inventories.iter().map(|inventory| inventory.iter().sum::<u32>()).collect::<Vec<u32>>();

    println!("{:#?}", inventory_totals);
    println!("{:#?}", inventory_totals.iter().max());
}