//use std::fs::File;
//use std::io::{BufRead,BufReader};
//use regex::Regex;
//use std::collections::{HashSet,HashMap};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day9a() {
        assert_eq!(32,day9a(9,25));
        assert_eq!(8317,day9a(10,1618));
        assert_eq!(146373,day9a(13,7999));
        assert_eq!(2764,day9a(17,1104));
        assert_eq!(54718,day9a(21,6111));
        assert_eq!(37305,day9a(30,5807));
    }

    #[test]
    fn test_day9b() {
        assert_eq!(0,day9b("input/07a_test1.txt"));
    }
}   

fn day9a(players: usize, count: usize) -> usize {
    let mut forward: Vec<usize> = Vec::new();
    let mut backward: Vec<usize> = Vec::new();
    let mut score: Vec<usize> = Vec::new();

    for _ in 0..count+1 {
        forward.push(0);
        backward.push(0);
    }

    for _ in 0..players {
        score.push(0);
    }

    let mut current = 0;
    let mut player = 0;
    for new_marble in 1..count+1 {
        if new_marble % 23 > 0 {
            current = forward[current];
            let next = forward[current];
            println!("Inserting {} after {}",current,new_marble);
            forward[current] = new_marble;
            forward[new_marble] = next;
            backward[next] = new_marble;
            backward[new_marble] = current;
            current = new_marble;
        } else {
            for _ in 0..7 {
                current = backward[current];
            }
            println!("Player {} scores {} and {}",player, new_marble, current);
            score[player] += new_marble + current;
            let next = forward[current];
            let prev = backward[current];
            forward[prev] = next;
            backward[next] = prev;
            current = next;
        }
        player = (player + 1) % players;
    }
    let mut maximum = 0;
    for player in 0..players {
        if score[player] > maximum {
            maximum = score[player];
        }
    }  
    maximum
}

    
fn day9b(_filename: &str) -> u32 {    
   0
}

pub fn run() {  
   println!("Part 1 answer: {:?}", day9a(412,71646));
   println!("Part 2 answer: {:?}", day9a(412,7164600)); 
}
