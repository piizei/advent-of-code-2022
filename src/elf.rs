#[derive(Debug)]
pub struct Elf {
    pub inventory: Vec<String>
}


impl Elf {
    pub fn calories(&self) -> u32 {
        // iterate over the inventory and sum the calories
        let mut sum = 0;
        for item in &self.inventory {
            let calories = item.parse::<u32>().unwrap();
            sum += calories;
        }
        return sum;
    }
}