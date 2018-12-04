use std::fs::File;
use std::io::{BufRead,BufReader};
use std::str;
use std::collections::HashMap;
use std::string::String;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day2a() {
        assert_eq!(12,day2a("input/02a_test1.txt"));
    }

    #[test]
    fn test_day2b() {
        assert_eq!("fgij",day2b("input/02b_test1.txt"))   
    }
}

fn get2_3(map: HashMap<char,u32>) -> (bool, bool) {
    let mut bool2 = false;
    let mut bool3 = false;

    for (_key, value) in map {
        if value == 2 {
            bool2 = true;
        } else if value == 3 {
            bool3 = true;
        }        
    }
    (bool2, bool3)
}

fn day2a(filename: &str) -> u32 {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);

    let mut sum2:u32 = 0;
    let mut sum3:u32 = 0;
   
    for line in f.lines() {
        let line = line.expect("Unable to read line");
      
        let mut map:HashMap<char,u32> = HashMap::new();

        for c in line.chars() {
            let value = map.entry(c).or_insert(0);
            *value += 1;
        }
        
        let (bool2, bool3) = get2_3(map);
    
        if bool2 == true {
            sum2 += 1; 
        } 
        if bool3 == true {
            sum3 += 1;
        }
    }

    sum2*sum3
}
    
fn day2b(filename: &str) -> std::string::String {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);

    let mut codes:Vec<String> = Vec::new();

    for code in f.lines() {
        let code = code.expect("Unable to read line");
      
        let length = code.len();
        for other in &codes {
            let mut mismatches = 0;
            let mut mismatch = 0;
            for pos in 0..length-1 {
                if code.chars().nth(pos).unwrap() != other.chars().nth(pos).unwrap() {
                    mismatches +=1;
                    mismatch = pos;
                }
            }
            if mismatches == 1 {
                // Found it
                return format!("{}{}",
                    code[..mismatch].to_string(),code[mismatch+1..].to_string());
            }
        }
        codes.push(code);
    }
    "fail".to_string()
}

pub fn run() {  
    println!("Part 1 answer: {:?}", day2a("input/02.txt"));
    println!("Part 2 answer: {:?}", day2b("input/02.txt")); 
}
