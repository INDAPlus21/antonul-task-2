/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 */

use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    // get input lines as integers
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let  values = input
        .lock()
        .lines()
        .next().unwrap().ok().unwrap()
        .split_whitespace()
        .map(|_value| _value.parse::<usize>().ok().unwrap())
        .collect::<Vec<usize>>();

    let rows: usize = values[0];
    let columns: usize = values[1];

    //for every row and every column
    for row in 1..=rows {   
    let mut row_output: String = String::default();
        for col in 1..=columns {
            let top_dist = row;
            let bot_dist = rows - row + 1;
            let left_dist = col;
            let right_dist = columns - col + 1;
            //gets the smallest value of ..._dist variables
            let min_distance = top_dist.min(bot_dist).min(left_dist).min(right_dist);
            //adds the distance if it's under 10 to output and . if it's not
            if min_distance < 10 {
                row_output.push_str(&min_distance.to_string());
            }
            else {
                row_output.push('.');
            }
        }
    println!("{}", row_output);
    }
}