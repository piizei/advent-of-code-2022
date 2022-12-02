mod elf;

use std::collections::HashMap;

use utils::read_rps_list;

use crate::utils::read_elf_list;
mod utils;


fn main() {
    // day1();
    day2();
}

fn day1() {
    let mut elfs = read_elf_list();
    
    for elf in &mut elfs {
        println!("Elf: {:?}", elf);  
        println!("Calories: {}", elf.calories());
    }
    let max:u32 = elfs.iter().map(|elf| elf.calories()).max().unwrap();
    println!("Best calories: {}", max);

    let mut calories:Vec<u32>  = elfs.iter().map(|elf| elf.calories()).collect();
    calories.sort();
    calories.reverse();
    println!("Best 3 calories: {}", calories[0]+calories[1]+calories[2]);
}

fn day2() {

    // X = A = Rock = 1
    // Y = B = Paper = 2
    // Z = C = Scissors = 3
    let rock = 1;
    let paper = 2;
    let scissors = 3;
    let win = 6;
    let lose = 0;
    let draw = 3;

    let mut comps = HashMap::new();
    comps.insert("A Y".to_string(), paper + win);    
    comps.insert("A X".to_string(), rock + draw);    
    comps.insert("A Z".to_string(), scissors + lose);        
    comps.insert("B Y".to_string(), paper + draw);
    comps.insert("B X".to_string(), rock + lose);
    comps.insert("B Z".to_string(), scissors + win);
    comps.insert("C Y".to_string(), paper + lose);
    comps.insert("C X".to_string(), rock + win);
    comps.insert("C Z".to_string(), scissors + draw);

    let rps_list = read_rps_list();
    let sum = rps_list.iter().map(|rps| comps.get(rps).unwrap()).sum::<u32>();
    println!("Sum: {}", sum);

    //Part 2
    comps.clear();
    comps.insert("A Y".to_string(), rock + draw);    
    comps.insert("A X".to_string(), scissors + lose);    
    comps.insert("A Z".to_string(), paper + win);        
    comps.insert("B Y".to_string(), paper + draw);
    comps.insert("B X".to_string(), rock + lose);
    comps.insert("B Z".to_string(), scissors + win);
    comps.insert("C Y".to_string(), scissors + draw);
    comps.insert("C X".to_string(), paper + lose);
    comps.insert("C Z".to_string(), rock + win);
    let sum = rps_list.iter().map(|rps| comps.get(rps).unwrap()).sum::<u32>();
    println!("Sum: {}", sum);
}

