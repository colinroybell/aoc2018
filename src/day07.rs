use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;
use std::char::from_u32;
//use std::collections::{HashSet,HashMap};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day7a() {
        assert_eq!("CABDFE".to_string(),day7a("input/07a_test1.txt"));
    }

    #[test]
    fn test_day7b() {
        // again different parameters 
        assert_eq!(15,day7b("input/07a_test1.txt",2,0));
    }
}   

fn day7a(filename: &str) -> String {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
    
    let mut deps: Vec<Vec<usize>> = Vec::new();
    let mut counts: Vec<isize> = Vec::new();
    let mut output: String = "".to_string();
    let mut seen: Vec<bool> = Vec::new();

    for _i in 0..27 {
        deps.push(Vec::new());
        counts.push(0);
        seen.push(false);
    }

    let re1 = Regex::new(r"Step (.) must be finished before step (.)").unwrap();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        assert!(re1.is_match(&line));
        let cap = re1.captures(&line).unwrap();
        let a = cap[1].to_string()[0..1].chars().next().unwrap() as usize -64;
        let b = cap[2].to_string()[0..1].chars().next().unwrap() as usize -64;     
        deps[a].push(b);
        counts[b]+=1;
        seen[a] = true;
        seen[b] = true;
    } 

    let mut done = false;

    while !done {
        done = true;
        for i in 0..27 {
            if counts[i]==0 && seen[i] {
                counts[i] = -1;
                output.push(from_u32(i as u32 + 64).expect("assembling ascii"));
                for j in &deps[i] {
                    counts[*j as usize] -=1;
                }
                done = false;
                break;
            }   
        }        
    }    
    output
}

    
fn day7b(filename: &str, elves: usize, offset: usize) -> u32 {    
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
    
    let mut deps: Vec<Vec<usize>> = Vec::new();
    let mut counts: Vec<isize> = Vec::new();
    let mut seen: Vec<bool> = Vec::new();
    let mut remaining: Vec<usize> = Vec::new();
    let mut assignment: Vec<isize> = Vec::new();
    let mut tasks_remaining = 0;
    let mut clock = 0;

    for _i in 0..27 {
        deps.push(Vec::new());
        counts.push(0);
        seen.push(false);
        remaining.push(0);
    }
    for _i in 0..elves {
        assignment.push(-1);
    }

    let re1 = Regex::new(r"Step (.) must be finished before step (.)").unwrap();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        assert!(re1.is_match(&line));
        let cap = re1.captures(&line).unwrap();
        let a = cap[1].to_string()[0..1].chars().next().unwrap() as usize -64;
        let b = cap[2].to_string()[0..1].chars().next().unwrap() as usize -64;     
        deps[a].push(b);
        counts[b]+=1;
        seen[a] = true;
        seen[b] = true;
    } 

    for i in 0..27 {
        if seen[i] {
            remaining[i] = offset + i;
            tasks_remaining += 1;
        }

    }

    while tasks_remaining > 0 {
        println!("clock {}",clock);
        for elf in 0..elves {
            let elf: usize = elf;
            let task = assignment[elf];
            if task >= 0 {
                let task = task as usize;
                remaining[task]-=1;
                if remaining[task] == 0 {
                    println!("task {} is done",task);
                    tasks_remaining -= 1;
                    for j in &deps[task] {
                        counts[*j as usize] -=1;
                    }
                    assignment[elf] = -1 // better to use None
                }
            }
            if assignment[elf] == -1 {
                for i in 0..27 {
                    if counts[i]==0 && seen[i] {
                        println!("elf {} starts task {}",elf,task);
                        counts[i] = -1;
                        assignment[elf] = i as isize;
                        break;
                    }   
                }        
            }
        }
        clock += 1
    }    
    clock - 1
}

pub fn run() {  
    println!("Part 1 answer: {:?}", day7a("input/07.txt"));
    println!("Part 2 answer: {:?}", day7b("input/07.txt", 5, 60)); 
}
