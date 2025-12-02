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
                        // if is_even_digit_length(i) {
                            let num_length = i.to_string().len();
                            for j in (1..=num_length).filter(|&d| num_length % d == 0) {
                                // println!("{} - {} - {}",i,num_length,j);

                                // for (exponent, digit_char) in i.to_string().chars().enumerate() {
                                //     println!("{:?}",digit_char);
                                // }                                
                                let chars: Vec<char> = i.to_string().chars().collect();
                                let mut chunk_counts: HashMap<String, usize> = HashMap::new();
                                
                                for chunk in chars.chunks(j) {
                                    let digits: String = chunk.iter().collect();
                                    *chunk_counts.entry(digits.clone()).or_insert(0) += 1;
                                    // println!("Chunk of {}: {}", j, digits);
                                }               
 

                                if let Some(&count) = chunk_counts.values().next() {
                                    if chunk_counts.len() == 1 && count > 1 {
                                        // println!("{}-{}",first_id, last_id);
                                        *found_numbers.entry(i.clone()).or_insert(0) += 1;
                                        // println!("{} - {} - {}",i,num_length,j);
                                        // println!("Chunks: {:?}", chunk_counts);   
                                        // println!("Length of Chunks: {}",chunk_counts.len());  
                                        // println!("Match!  <---------------------------");
                                    }
                                }   
                                
                            }

                            // println!("----");
                            // for (exponent, digit_char) in i.to_string().chars().enumerate().skip(2) {
                            //     println!("{:?}",digit_char);
                            // }                            
                        // }
                    }
                }
            }  
            // break;
        }

        // println!("found_numbers: {:?}", found_numbers); 
        let mut sum:i64 = 0;
        for entry in found_numbers {
            println!("{:?}",entry);
            // if entry.1 > 1 {
            sum += entry.0;
            // }
        }
        println!("sum: {:?}", sum); 
    }



    // println!("Counter: {}", counter);
    
    Ok(())
}
