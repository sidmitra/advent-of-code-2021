#![allow(unused)]

extern crate colored;
use colored::*;

use std::fs;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// 1604: too high
fn main() {
    let filename: &str = "input.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = content.lines().collect::<Vec<&str>>();
    const RADIX: u32 = 10;

    let mut cave: Vec<Vec<u32>> = Vec::new();
    for (idy, line) in lines.iter().enumerate() {
        let mut row: Vec<u32> = Vec::new();
        let chars = line.chars().map(|c| c.to_digit(RADIX).unwrap());
        // println!("{:?}", line);
        // println!("{:?}", chars);
        for (_idx, ch) in chars.enumerate() {
            // print_type_of(&ch);
            row.push(ch);
        }
        cave.push(row);
    }

    let height: usize = cave.len();
    let width: usize = cave[0].len();

    // Print
    // for h in 0..height {
    //     for w in 0..width {
    //         print!("{:?}", cave[h][w]);
    //     }
    //     println!("");
    // }
    // println!("=======");

    let mut debug = false;
    let mut risk_level = 0;
    for h in 0..height {
        for w in 0..width {
            let mut is_low = true;

            // if (h == 0 && w == 9) {
            //     debug = true;
            //     println!("---")
            // }
            // else{
            //     debug = false;
            // }

            // Check left
            if (w >= 1) {
                if debug {
                    println!("left: within bounds {:?}", w);
                }

                if cave[h][w - 1] <= cave[h][w] {
                    is_low = false;
                }
            }

            // Check right
            if (w < width - 1) {
                if debug {
                    println!("right: within bounds {:?}", w);
                }
                if (cave[h][w + 1] <= cave[h][w]) {
                    // is_right_higher = true
                    is_low = false;
                    if debug{
                        println!("2: low false");
                    }
                }
            }

            // Check top
            if (h >= 1) {
                if debug {
                    println!("top: within bounds {:?}", h);
                }
                if (cave[h - 1][w] <=cave[h][w]) {
                    is_low = false;
                }
            }

            // Check bottom
            if (h < height - 1) {
                if debug {
                    println!("bottom: within bounds {:?}", h);
                }
                if (cave[h + 1][w] <= cave[h][w]) {
                    is_low = false;
                }
            }

            if debug{
                println!("{:?}, {:?}: {:?} = {:?}", h, w, is_low, cave[h][w]);
            }
            if is_low {
                // println!("{:?}, {:?}, risk={:?}", h, w, cave[h][w]);
                risk_level = risk_level + 1 + cave[h][w];
                print!("{:}", cave[h][w].to_string().green());
            }
            else{
                print!("{:?}", cave[h][w]);
            }
        }
        println!("");
    }
    println!("{}", risk_level.to_string().green())
}
