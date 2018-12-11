use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;
use std::collections::{BTreeMap,HashMap};


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day4a() {
        assert_eq!(240,day4a("input/04a_test1.txt"))
    }

    #[test]
    fn test_day4b() {
        assert_eq!(4455,day4b("input/04a_test1.txt"))
    }
}   


fn day4a(filename: &str) -> u32 {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
 

    let re1 = Regex::new(r"\[(.+)\] (.+)").unwrap();
    let mut map:BTreeMap<String,String> = BTreeMap::new();


    for line in f.lines() {
        let line = line.expect("Unable to read line");
        assert!(re1.is_match(&line));
        let cap = re1.captures(&line).unwrap();
        let key = cap[1].to_string();
        let val = cap[2].to_string();     
        map.insert(key,val);
    }

    let re2 = Regex::new(r"(\d+)").unwrap();
    let mut guard:u32 = 0;
    let mut asleep:u32 = 0;
    let mut awake:u32;
    let mut map2:HashMap<u32,u32> = HashMap::new();
    for (key,value) in &map {
        let time = key[14..].parse().unwrap();
        println!("{} {}",key,time);
        if re2.is_match(&value) {
            let cap = re2.captures(&value).unwrap();
            guard = cap[1].parse().unwrap();
        } else if value[0..5].to_string() == "falls" {
            asleep = time;
        } else {
            awake = time;
            let value = map2.entry(guard).or_insert(0);
            *value += awake-asleep;
        }    
    }
    let mut chosen_guard:u32 = 0;
    let mut maximum:u32 = 0;
    for (key,value) in map2 {
        if value > maximum {
            chosen_guard = key;
            maximum = value;
        }
    }

    let mut sleeps:[u32;60] = [0;60];
    for (key,value) in &map {
        let time = key[14..].parse().unwrap();
        if re2.is_match(&value) {
            let cap = re2.captures(&value).unwrap();
            guard = cap[1].parse().unwrap();
        } else if value[0..5].to_string() == "falls" {
            asleep = time;
        } else {
            awake = time;
            if guard == chosen_guard {
                println!("{} {}",asleep,awake);
                for t in asleep..awake {
                    sleeps[t as usize] += 1;
                }
            }
        }    
    }
    
    let mut chosen_time:u32 = 0;
    let mut maximum:u32 = 0;

    for t in 0..60 {
        println!("{} {}",t,sleeps[t as usize]);
        if sleeps[t as usize] > maximum {
            maximum = sleeps[t as usize];
            chosen_time = t;
        }
    }

    chosen_guard * chosen_time
}

    
fn day4b(filename: &str) -> u32 {    
     let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
 

    let re1 = Regex::new(r"\[(.+)\] (.+)").unwrap();
    let mut map:BTreeMap<String,String> = BTreeMap::new();


    for line in f.lines() {
        let line = line.expect("Unable to read line");
        assert!(re1.is_match(&line));
        let cap = re1.captures(&line).unwrap();
        let key = cap[1].to_string();
        let val = cap[2].to_string();     
        map.insert(key,val);
    }

    let re2 = Regex::new(r"(\d+)").unwrap();
    let mut guard:u32 = 0;
    let mut asleep:u32 = 0;
    let mut awake:u32;
    let mut map2:HashMap<String,u32> = HashMap::new();
    for (key,value) in &map {
        let time = key[14..].parse().unwrap();
        println!("{} {}",key,time);
        if re2.is_match(&value) {
            let cap = re2.captures(&value).unwrap();
            guard = cap[1].parse().unwrap();
        } else if value[0..5].to_string() == "falls" {
            asleep = time;
        } else {
            awake = time;
            for t in asleep..awake {
                let string = format!("{} {}",guard,t);
                let v = map2.entry(string).or_insert(0);
                *v += 1
            }
        }    
    }

    let mut chosen_guard:u32 = 0;
    let mut chosen_time:u32 = 0;
    let mut maximum:u32 = 0;
    let re3 = Regex::new(r"(\d+) (\d+)").unwrap();
    for (key,value) in map2 {
        if value > maximum {
            let cap = re3.captures(&key).unwrap();
            chosen_guard = cap[1].parse().unwrap();
            chosen_time = cap[2].parse().unwrap();
            maximum = value;
        }
    }
    chosen_guard * chosen_time
}

pub fn run() {  
    println!("Part 1 answer: {:?}", day4a("input/04.txt"));
    println!("Part 2 answer: {:?}", day4b("input/04.txt")); 
}
