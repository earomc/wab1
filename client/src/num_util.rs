// Ausgeborgt von https://codereview.stackexchange.com/questions/173338/calculate-mean-median-and-mode-in-rust
// Für eigene Zwecke modifiziert

pub fn average(numbers: &Vec<u128>) -> f64 {
    numbers.iter().sum::<u128>() as f64 / numbers.len() as f64
}

pub fn median(numbers: &mut Vec<u128>) -> u128 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}