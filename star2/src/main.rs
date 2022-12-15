fn main() {
    let mut elves: Vec<i32> = Vec::new();
    loop {
        let elf_val = read_elf();
        if elf_val == -1 {
            break;
        }
        elves.push(elf_val);
    }

    let mut result = 0;
    
    for _i in 0..3 {
        let max = elves.iter().max().unwrap();
        let max_index = elves.iter().fold((0, 0), |(ind, result), x| if x == max {(ind+1, ind)} else {(ind+1, result)}).1;

        result += max;
        elves.remove(max_index);
    }
    println!("{}", result);
}

/* Read a single elf, i.e. read lines until an empty one is found, then return the sum

Return: amount of calories of this elf, if it encounters "end" then return -1.

Lire un seul elfe, soit lire des lignes jusqu'à ce que une ligne vide soit trouver, puis return la somme

Return: nobre de calories de cet elfe, si "end" est lu, retourne -1.
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