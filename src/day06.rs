use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;
use std::collections::{HashSet,HashMap};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day6a() {
        assert_eq!(17,day6a("input/06a_test1.txt"));
    }

    #[test]
    fn test_day6b() {
        // won't work as we need different value of threshold
//        assert_eq!(16,day6b("input/06a_test1.txt"));
    }
}   

fn tryNew(nextClaims: &mut HashMap<(usize,usize),usize>, claims: &mut HashMap<(usize,usize),usize>, n: usize, contested: usize, x: usize, y: usize) -> () {
    let key = (x,y);
    if claims.contains_key(&key) {
        return
    }
    let claim = nextClaims.entry(key).or_insert(n);
    if *claim != n {
        *claim = contested;
    }
}

fn day6a(filename: &str) -> usize {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
 
    let re1 = Regex::new(r"(\d+),\s+(\d+)").unwrap();
    
    let mut minX=1000000;
    let mut minY=1000000;
    let mut maxX=0;
    let mut maxY=0;

    let mut infinite: HashSet<usize> = HashSet::new();
    let mut claims: HashMap<(usize,usize),usize> = HashMap::new();
    let mut newClaims: HashMap<(usize,usize),usize> = HashMap::new();
    let mut nextClaims: HashMap<(usize,usize),usize> = HashMap::new();
    let mut counts:HashMap<usize,usize> = HashMap::new();

    let mut num = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        assert!(re1.is_match(&line));
        let cap = re1.captures(&line).unwrap();
        let x = cap[1].parse().unwrap();
        let y = cap[2].parse().unwrap();     
        newClaims.insert((x,y),num);
        counts.insert(num,0);
        if x < minX {
            minX = x
        } 
        if x > maxX {
            maxX = x
        }
        if y < minY {
            minY = y
        }
        if y > maxY {
            maxY = y
        }
        num+=1
     }

     
     while newClaims.len() > 0 {
        for (pos,n) in newClaims.iter() {
            let n = *n;
            claims.insert(*pos,n);
            let val = counts.entry(n).or_insert(0);
            *val += 1;
            println!("{} claims {},{}",n,pos.0,pos.1);
            if pos.0 == minX || pos.0 == maxX || 
                pos.1 == minY || pos.1 == maxY {
                println!("{}","infinite");
                infinite.insert(n);
            }
        }
        for (pos,n) in newClaims.iter() {
            let n = *n;
            // FIXME: claims is not being changed but can't see a way to pass otherwise
            if pos.0 < maxX {
                tryNew(&mut nextClaims,&mut claims,n,num,pos.0+1,pos.1+0);
            }
            if pos.0 > minX {
                tryNew(&mut nextClaims,&mut claims,n,num,pos.0-1,pos.1+0);
            }
            if pos.1 < maxY {
                tryNew(&mut nextClaims,&mut claims,n,num,pos.0+0,pos.1+1);
            }
            if pos.1 > minY {
                tryNew(&mut nextClaims,&mut claims,n,num,pos.0+0,pos.1-1);
            }
        }
        newClaims.clear();
        for (pos,n) in nextClaims.iter() {
            newClaims.insert(*pos,*n);
        }
        nextClaims.clear();
    }
    let mut maxSize: usize = 0;
    for n in 0..num {
        if !infinite.contains(&n) {
            let size = counts.get(&n).unwrap();
            if *size > maxSize {
                maxSize = *size;
            }
        }
    }
    maxSize

}

    
fn day6b(filename: &str) -> usize {    
      let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
 
     let mut cores :Vec<(isize,isize)> = Vec::new();

     let re1 = Regex::new(r"(\d+),\s+(\d+)").unwrap();
   

     for line in f.lines() {
        let line = line.expect("Unable to read line");
        assert!(re1.is_match(&line));
        let cap = re1.captures(&line).unwrap();
        let x = cap[1].parse().unwrap();
        let y = cap[2].parse().unwrap();     
        cores.push((x,y));
     }

     let mut count :usize = 0;
     for x in 0..400 {
        for y in 0..400 {
            let mut sum :isize = 0;
            for core in &cores {
                sum+=(x-core.0).abs()+(y-core.1).abs()
            }
            if sum < 10000 {
                count+=1
            }
        }
     }
     count

}

pub fn run() {  
    println!("Part 1 answer: {:?}", day6a("input/06.txt"));
    println!("Part 2 answer: {:?}", day6b("input/06.txt")); 
}
