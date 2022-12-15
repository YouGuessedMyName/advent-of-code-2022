fn main() {
    let mut result = 0;
    loop {
        let elf_val = read_elf();
        if elf_val == -1 {
            break;
        }
        if elf_val > result {
            result = elf_val;
        }
    }
    println!("{}", result);
}

/* Read a single elf, i.e. read lines until an empty one is found, then return the sum

Return: amount of calories of this elf.

Lire un seul elfe, soit lire des lignes jusqu'à ce que une ligne vide soit trouver, puis return la somme

Return: nobre de calories de cet elfe.
*/
fn read_elf() -> i32 {
    let mut calorie_count: i32 = 0;
    loop {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .unwrap();
        // The string input is now read to line
        if line.trim().is_empty() { // Break when nothing is on the input
            break;
        }
        else if line.trim() == "end" {
            calorie_count = -1;
            break;
        }
        else { // Parse an integer if the input is valid
            let calories: i32 = line
                .trim()
                .parse()
                .expect("Corrupted input");
            calorie_count += calories;
        }
    }
    calorie_count
}
