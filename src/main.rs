mod elf;
mod stacks;
mod stream_scanner;

use std::collections::HashMap;

use stream_scanner::Scanner;
use utils::{read_rps_list, get_day_6};

use crate::utils::{read_elf_list, read_moves_list, parse_moves, get_day_5_stacks};
mod utils;


fn main() {
    // day1();
    //day2();
    //day5_1();
    //day5_2();
    //day6();
    day6_2();
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


fn day5_1() {
    // Stacks for the cranes
    let mut stacks = get_day_5_stacks();

    let moves = read_moves_list();

    for m in moves {
        let amount = m[0];
        let from = m[1];
        let to = m[2];
        println!("Amount: {:?}", amount);
        println!("from: {:?}", from);
        println!("to: {:?}", to);
        for n in 1..=amount {    
            println!("Round: {:?}", n);        
            let c = stacks.get_mut(&from).unwrap().pop().unwrap();
            println!("C: {:?}", c);
            stacks.get_mut(&to).unwrap().push(c);
        }   

    }

    println!("Stack: {:?}", stacks.get_mut(&1).unwrap().pop().unwrap());
    println!("Stack: {:?}", stacks.get_mut(&2).unwrap().pop().unwrap());
    println!("Stack: {:?}", stacks.get_mut(&3).unwrap().pop().unwrap());
    println!("Stack: {:?}", stacks.get_mut(&4).unwrap().pop().unwrap());
    println!("Stack: {:?}", stacks.get_mut(&5).unwrap().pop().unwrap());
    println!("Stack: {:?}", stacks.get_mut(&6).unwrap().pop().unwrap());
    println!("Stack: {:?}", stacks.get_mut(&7).unwrap().pop().unwrap());
    println!("Stack: {:?}", stacks.get_mut(&8).unwrap().pop().unwrap());
    println!("Stack: {:?}", stacks.get_mut(&9).unwrap().pop().unwrap());

}

fn day5_2() {
    // Stacks for the cranes
    let mut stacks = get_day_5_stacks();

    let moves = read_moves_list();

    for m in moves {
        let amount = m[0];
        let from = m[1];
        let to = m[2];
        let mut origin_stack = stacks.get(&from).unwrap().clone();
        let mut target_stack = stacks.get(&to).unwrap().clone();  
        let range = origin_stack.split_off(origin_stack.len() - amount as usize) ;
        target_stack.extend(range.iter());
        stacks.insert(from, origin_stack);
        stacks.insert(to, target_stack);


    }

    println!("Stack: {:?}", stacks.get_mut(&1).unwrap().pop().unwrap());
    println!("Stack: {:?}", stacks.get_mut(&2).unwrap().pop().unwrap());
    println!("Stack: {:?}", stacks.get_mut(&3).unwrap().pop().unwrap());
    println!("Stack: {:?}", stacks.get_mut(&4).unwrap().pop().unwrap());
    println!("Stack: {:?}", stacks.get_mut(&5).unwrap().pop().unwrap());
    println!("Stack: {:?}", stacks.get_mut(&6).unwrap().pop().unwrap());
    println!("Stack: {:?}", stacks.get_mut(&7).unwrap().pop().unwrap());
    println!("Stack: {:?}", stacks.get_mut(&8).unwrap().pop().unwrap());
    println!("Stack: {:?}", stacks.get_mut(&9).unwrap().pop().unwrap());

}

fn day6() {
    let mut scanner = Scanner { buffer: vec!['m'], buffer_length: 4 };
    let mut count = 0;
    for char in get_day_6() {
        count += 1;
        scanner.add(char);
        if (scanner.unique()) {
            println!("index: {}", count);
            println!("buffer: {:?}", scanner.buffer);
            return;
        }

    }
}

fn day6_2() {
    let mut scanner = Scanner { buffer: vec!['m'], buffer_length: 14 };
    let mut count = 0;
    for char in get_day_6() {
        count += 1;
        scanner.add(char);
        if (scanner.unique()) {
            println!("index: {}", count);
            println!("buffer: {:?}", scanner.buffer);
            return;
        }

    }
}
