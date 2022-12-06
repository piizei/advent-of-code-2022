use std::collections::HashSet;

#[derive(Debug)]
pub struct Scanner {
    pub buffer: Vec<char>,
    pub buffer_length: usize,
}


impl Scanner {
    pub fn unique(&self) -> bool {
        // iterate over the inventory and sum the calories
        {
            if (self.buffer.len() < self.buffer_length) {
                return false;
            }
            let mut uniq = HashSet::new();
            self.buffer.clone().into_iter().all(move |x| uniq.insert(x))
        }
    }
    pub fn add(&mut self, c: char) {
        self.buffer.push(c);
        if (self.buffer.len() == self.buffer_length + 1) {
            self.buffer.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn uniqueness() {
        //Unique if less than 4 letters:
        let mut scanner = Scanner { buffer: vec!['a', 'b', 'c'], buffer_length: 4 };
        assert_eq!(scanner.unique(), false);
        let mut scanner = Scanner { buffer: vec!['a', 'b', 'c', 'd'], buffer_length: 4 };
        assert_eq!(scanner.unique(), true);
        scanner.buffer.push('a');
        assert_eq!(scanner.unique(), false);
    }
    #[test]
    fn length() {
        let mut scanner = Scanner { buffer: vec!['a', 'b', 'c', 'd'], buffer_length: 4 };
        scanner.add('e');
        assert_eq!(scanner.buffer.len(), 4); 
    }
}