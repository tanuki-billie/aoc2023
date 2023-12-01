use std::io;
use regex::Regex;

// Day 1 - For each line, get the first and last digits and return the sum
fn main() {
    let mut result: u64 = 0;
    let digit_re = Regex::new(r"[\d]").unwrap();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("readline failed");
        let input = input.trim();
        // It's over
        if input.is_empty() {
            break;
        }
        // Convert words to digits
        let input = input
            .replace("one", "on1e")
            .replace("two", "tw2o")
            .replace("three", "thre3e")
            .replace("four", "fou4r")
            .replace("five", "fiv5e")
            .replace("six", "si6x")
            .replace("seven", "seve7n")
            .replace("eight", "eigh8t")
            .replace("nine", "nin9e");
        // Get first/last digits and add to result
        let digits: Vec<_> = digit_re.find_iter(&input).map(|m| m.as_str()).collect();
        let digits = (*digits.first().unwrap(), *digits.last().unwrap());
        let digits = digits.0.to_owned() + digits.1;
        let digits = digits.parse::<u64>().unwrap();
        result += digits;
        println!("{}", digits);
    }
    println!("The result is {}", result);
}