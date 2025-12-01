use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;

    let reader = BufReader::new(file);

    let pattern = Regex::new(r"(?P<direction>[A-Z])(?P<num1>\d+)").unwrap();

    let mut current_value = 50;
    let mut counter = 0;

    for line in reader.lines() {
        let text = line?;
        println!("{}", text);     

        for caps in pattern.captures_iter(&text) {
            if let Some(dir_match) = caps.name("direction") {
                let direction = dir_match.as_str();
                // println!("Matched: {}", direction);
                let num1: i32 = caps["num1"].parse().unwrap();
                println!("Current: {}",current_value);


                if direction == "L" {
                    println!("Left: {}",num1);
                    let result = ((current_value - num1) % 100 + 100) % 100;
                    println!("Result: {}",result);
                    if result == 0 {
                        counter += 1;
                    }                    
                    current_value = result;
                } else {
                    println!("Right: {}",num1);
                    let result = ((current_value + num1) % 100 + 100) % 100;
                    println!("Result: {}",result); 
                    if result == 0 {
                        counter += 1;
                    }                          
                    current_value = result;             
                }
            }
        }   
    }

    println!("Counter: {}", counter);
    
    Ok(())
}
