use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

extern crate regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(day) = args[1].parse::<i32>().ok() {
        match day {
            1 => day01::run(),
            2 => day02::run(),
            3 => day03::run(),
            4 => day04::run(),
            5 => day05::run(),
            6 => day06::run(),
            7 => day07::run(),
            _ => println!("Can't run day {}",day),
        } 
    } else {
        println!("Day is invalid");
    }
}
