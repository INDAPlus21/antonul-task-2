//Shoutout Violas kod i discord

use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();

    let mut numbers: Vec<usize> = input
        .lock()
        .lines()   
        .skip(1) // read only number line
        .next().unwrap().ok().unwrap()
        .split_whitespace()
        .map(|_number| _number.parse::<usize>().ok().unwrap())
        .collect();

    let mut sum = 0;

    match numbers.len() {
        // edge cases, logic and print
        0 => {
            sum = 0;
        }
        _ => {
            numbers.sort();
            let half_of_numbers = numbers.len()/2;
            for number in half_of_numbers..numbers.len() {
                sum += numbers[number];
            }
        }
    }
    println!("{}", sum);
}