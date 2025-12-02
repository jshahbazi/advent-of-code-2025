use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;

fn is_even_digit_length(num: i64) -> bool {
    num.to_string().len() % 2 == 0
}


fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;

    let reader = BufReader::new(file);

    let pattern = Regex::new(r"(?P<num1>\d+)-(?P<num2>\d+)").unwrap();

    // let mut counter = 0;

    for line in reader.lines() {
        let text = line?;
        // println!("{}",text);
        // println!("");

        let parts: Vec<&str> = text.split(",").collect();

        let mut found_numbers: HashMap<i64, usize> = HashMap::new();

        // println!("{:?}", parts);
        // println!("");
        for part in parts {
            for caps in pattern.captures_iter(part) {
                if let (Some(_n1), Some(_n2)) = (caps.name("num1"), caps.name("num2")) {
                    let first_id: i64 = caps["num1"].parse().unwrap();
                    let last_id: i64 = caps["num2"].parse().unwrap();
                    println!("{}-{}",first_id, last_id);

                    for i in first_id..=last_id {
                        if is_even_digit_length(i) {
                            let num_length = i.to_string().len();
                            let half = num_length / 2;
                            
                            let chars: Vec<char> = i.to_string().chars().collect();
                            let first_half: String = chars[..half].iter().collect();
                            let second_half: String = chars[half..].iter().collect();
                            
                            if first_half == second_half {
                                *found_numbers.entry(i).or_insert(0) += 1;
                                println!("{} - halves: {} | {}", i, first_half, second_half);
                                println!("Match!  <---------------------------");
                            }
                        }
                    }
                }
            }  
            // break;
        }

        // println!("found_numbers: {:?}", found_numbers); 
        println!("-----------");
        let mut sum:i64 = 0;
        for entry in found_numbers {
            println!("{:?}",entry.0);
            sum += entry.0;
        }
        println!("sum: {:?}", sum); 
    }



    // println!("Counter: {}", counter);
    
    Ok(())
}
