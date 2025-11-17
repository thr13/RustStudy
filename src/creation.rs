use std::collections::HashSet;
use rand::Rng;

pub fn number_creation() -> [u32; 6] {
    let mut rng = rand::rng();
    let mut random_numbers = HashSet::new();

    while random_numbers.len() < 6 {
        let random = rng.random_range(1..=45);
        random_numbers.insert(random);
    }

    let mut numbers: [u32; 6] = [0; 6];
    let mut i = 0;
    for &random in &random_numbers {
        numbers[i] = random;
        i += 1;
    }

    numbers
}