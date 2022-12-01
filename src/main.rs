use std::fs;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let elf_inventories = contents.split("\n\n");

    let mut elf_totals = Vec::new();

    for elf_inventory in elf_inventories {
        let elf_calorie_strings = elf_inventory.split("\n");
        let mut elf_total = 0;
        for elf_calorie_string in elf_calorie_strings {
            let elf_calorie = elf_calorie_string.parse::<i32>().unwrap();
            elf_total += elf_calorie;
        }
        elf_totals.push(elf_total);
    }

    let mut max_elf_total = 0;
    for elf_total in elf_totals {
        if elf_total > max_elf_total {
            max_elf_total = elf_total;
        }
    }

    println!("Max elf total: {}", max_elf_total);
}