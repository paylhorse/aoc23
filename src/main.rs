// ------ DAY 1 ------

use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

fn main() {
    let mut total: i32 = 0;
    let input = fs::read_to_string("input.txt").unwrap();
    for text_line in input.lines() {
        let mut word_num_map: HashMap<usize, usize> = HashMap::new(); // index, value
        let word_num_patterns = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        for i in 0..word_num_patterns.len() {
            let mut text_line_string = text_line.to_string();
            let mut dupe_counter = 0;
            // Loop so we can hit all duplicates
            loop {
                println!("Searching for: {}", word_num_patterns[i]);
                if let Some(index) = text_line_string.find(word_num_patterns[i]) {
                    word_num_map.insert(index + dupe_counter, i + 1);
                    // Hacky, nullify the numerical word by deleting the first character
                    text_line_string.remove(index);
                    dupe_counter += 1;
                } else { break }
            }
        }
        println!("{:?}", word_num_map);
        let min_index = word_num_map.keys().min();
        println!("MIN INDEX: {:?}", min_index);
        let max_index = word_num_map.keys().max();
        println!("MAX INDEX: {:?}", max_index);
        // -----------------------
        // 'a' so that we trigger a panic if something goes wrong downstream ;)
        let mut first: char = 'a';
        let mut last: char = 'a';
        // First assume that the first and last numerical words found are the solution
        if let Some(min_index) = min_index {
            first = char::from_digit(*word_num_map.get(min_index).unwrap() as u32, 10).unwrap();
        }
        if let Some(max_index) = max_index {
            last = char::from_digit(*word_num_map.get(max_index).unwrap() as u32, 10).unwrap();
        }
        // Now we check for numerals, compare to the index of numerical words to see if we should update the solution
        for (index, char) in text_line.char_indices() {
            if char.is_ascii_digit() {
                if let Some(min_index) = min_index {
                    if index < *min_index {
                        first = char;
                        break;
                    }
                } else {
                    first = char;
                    break;
                }
            }
        }
        for (index, char) in text_line.char_indices().rev() {
            if char.is_ascii_digit() {
                if let Some(max_index) = max_index {
                    if index > *max_index {
                        last = char;
                        break;
                    }
                } else {
                    last = char;
                    break;
                }
            }
        }
        let value_string = format!{"{}{}", first, last};
        let value = i32::from_str(&value_string).expect("Failed to convert char to i32!");
        println!("LINE VALUE: {}", value);
        total += value;
    }
    // DONE!
    println!{"FINAL SUM: {}", total};
}
