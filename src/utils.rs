use std::{fs, collections::HashMap};
use crate::elf::Elf;

pub fn read_elf_list() -> Vec<Elf> {
    let contents =
    fs::read_to_string("./input-day-1.txt").expect("Should have been able to read the file");
    let split = contents.split("\n\n");
    let elfs = split.map(|x| x.split("\n")).collect::<Vec<_>>();

    let mut response:Vec<Elf> = Vec::new();

    for elf in elfs {
        response.push( Elf {
            inventory: elf.map(|s| s.to_string()).collect()
        });        
    }

    return response;
}

pub fn read_rps_list() -> Vec<String> {
    let contents =
    fs::read_to_string("./input-day-2.txt").expect("Should have been able to read the file");
    return contents.split("\n").map(|s| s.to_string()).collect::<Vec<_>>();
}

pub fn read_moves_list() -> Vec<Vec<u32>> {
    let contents =
    fs::read_to_string("./input-day-5.txt").expect("Should have been able to read the file");
    return contents.lines().map(|l| parse_moves(l)).collect::<Vec<_>>();
}

pub fn parse_moves(input: &str) -> Vec<u32> {
    return input.split(" ").map(|i| i.parse::<u32>().unwrap()).collect::<Vec<_>>();
}

pub fn get_day_5_stacks() -> HashMap<u32, Vec<char>> {
    let mut stacks:HashMap<u32, Vec<char>> = HashMap::new();
    stacks.insert(1, "LNWTD".chars().collect());
    stacks.insert(2, "CPH".chars().collect());
    stacks.insert(3, "WPHNDGMJ".chars().collect());
    stacks.insert(4, "CWSNTQL".chars().collect());
    stacks.insert(5, "PHCN".chars().collect());
    stacks.insert(6, "THNDMWQB".chars().collect());
    stacks.insert(7, "MBRJGSL".chars().collect());
    stacks.insert(8, "ZNWGVBRT".chars().collect());
    stacks.insert(9, "WGDNPL".chars().collect());
    return stacks;
}

pub fn get_day_6() ->Vec<char> {
    let message = fs::read_to_string("./input-day-6.txt").expect("Should have been able to read the file");
    return message.chars().collect();
}