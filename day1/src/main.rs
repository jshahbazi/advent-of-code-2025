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
                let num1: i32 = caps["num1"].parse().unwrap();
                println!("Current: {}",current_value);

 
                if direction == "L" {
                    println!("Left: {}",num1);
                    let new_pos = current_value - num1;
                    if new_pos < 0 {
                        let crossings = if current_value == 0 {
                            (-new_pos) / 100
                        } else {
                            (-new_pos) / 100 + 1
                        };
                        if crossings > 0 {
                            println!("crossings: {}", crossings); 
                            counter += crossings;
                        }
                    } else if new_pos == 0 && current_value != 0 {
                        println!("crossings: 1"); 
                        counter += 1;
                    }
                    let result = ((current_value - num1) % 100 + 100) % 100;
                    println!("Result: {}",result);
               
                    current_value = result;
                } else {
                    println!("Right: {}",num1);
                    if current_value + num1 > 99 {
                        let crossings = (current_value + num1) / 100;
                        println!("crossings: {}", crossings);     
                        counter += crossings;
                    }                       
                    let result = ((current_value + num1) % 100 + 100) % 100;
                    println!("Result: {}",result); 
                        
                    current_value = result;             
                }
            }
        }   
    }

    println!("Counter: {}", counter);
    
    Ok(())
}
