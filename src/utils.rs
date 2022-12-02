use std::fs;
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