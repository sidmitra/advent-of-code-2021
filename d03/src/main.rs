#![allow(unused)]
use std::collections::HashMap;
use std::fs;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn part_one() {
    let filename = "input.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let rows: Vec<&str> = content.lines().collect();
    let word_length = rows[0].len();
    let mut zero_counts = vec![0; word_length];
    let mut one_counts = vec![0; word_length];

    for row in rows {
        // println!("row: {:?}", row);
        let mut frequency: HashMap<char, i32> = HashMap::new();
        let columns: Vec<char> = row.chars().collect();
        // println!("{:?}", columns);
        for (idx, column) in columns.iter().enumerate() {
            let key = row.chars().nth(idx).unwrap();
            // print_type_of(&key);
            *frequency.entry(key).or_insert(0) += 1;
            if column == &'0' {
                zero_counts[idx] += 1;
            } else {
                one_counts[idx] += 1;
            }
        }
    }

    // println!("{:?}", zero_counts);
    // println!("{:?}", one_counts);

    let mut gamma: String = "".to_owned();
    let mut epsilon: String = "".to_owned();
    for idx in 0..zero_counts.len() {
        if (zero_counts[idx] > one_counts[idx]) {
            gamma.push_str("0");
            epsilon.push_str("1");
        } else {
            gamma.push_str("1");
            epsilon.push_str("0");
        }
    }

    let gamma_int = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_int = isize::from_str_radix(&epsilon, 2).unwrap();
    let power_consumption = gamma_int * epsilon_int;
    println!(
        "{:?}*{:?} = {:?}",
        gamma_int, epsilon_int, power_consumption
    );
}

// Frequency count of 0s and 1s
fn count(rows: &Vec<&str>) -> (Vec<usize>, Vec<usize>) {
    let word_length = rows[0].len();

    // Calculate counts of 0s/1s in bit positions
    let mut zero_counts = vec![0; word_length];
    let mut one_counts = vec![0; word_length];
    for row in rows {
        // println!("row: {:?}", row);
        let mut frequency: HashMap<char, i32> = HashMap::new();
        let columns: Vec<char> = row.chars().collect();
        // println!("{:?}", columns);
        for (idx, column) in columns.iter().enumerate() {
            let key = row.chars().nth(idx).unwrap();
            // print_type_of(&key);
            *frequency.entry(key).or_insert(0) += 1;
            if column == &'0' {
                zero_counts[idx] += 1;
            } else {
                one_counts[idx] += 1;
            }
        }
    }
    return (zero_counts, one_counts);
}

fn calc_oxygen_rating(filename: &str) -> isize {
    // let filename = "input_test.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut rows: Vec<&str> = content.lines().collect();

    let word_length = rows[0].len();

    let mut rating: String = "".to_owned();
    for idx in 0..word_length {
        let (zero_counts, one_counts) = count(&rows);

        // there are more 1s in the bit position
        if (one_counts[idx] >= zero_counts[idx]) {
            // println!("more 1 //// {:?}", idx);
            rating.push_str("1");
            rows = rows
                .into_iter()
                .filter(|element| element.chars().nth(idx).unwrap() == '1')
                .collect();
        } else {
            // println!("more 0 ////{:?}", idx);
            rating.push_str("0");
            rows = rows
                .into_iter()
                .filter(|element| element.chars().nth(idx).unwrap() == '0')
                .collect();
        }
        if rows.len() == 1 {
            //return rows[0];
            break;
        }
    }

    let rating_int = isize::from_str_radix(&rows[0], 2).unwrap();
    println!("{:?}", rating_int);
    return rating_int;
}

fn calc_co2_rating(filename: &str) -> isize {
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut rows: Vec<&str> = content.lines().collect();

    let word_length = rows[0].len();

    let mut rating: String = "".to_owned();
    for idx in 0..word_length {
        let (zero_counts, one_counts) = count(&rows);

        // there are more 1s in the bit position
        if (one_counts[idx] >= zero_counts[idx]) {
            rows = rows
                .into_iter()
                .filter(|element| element.chars().nth(idx).unwrap() == '0')
                .collect();
        } else {
            rows = rows
                .into_iter()
                .filter(|element| element.chars().nth(idx).unwrap() == '1')
                .collect();
        }
        if rows.len() == 1 {
            break;
        }
    }

    let rating_int = isize::from_str_radix(&rows[0], 2).unwrap();
    println!("{:?}", rating_int);
    return rating_int;
}

fn part_two(filename: &str) {
    let oxygen_rating = calc_oxygen_rating(filename);
    let co2_rating = calc_co2_rating(filename);
    let life_support = oxygen_rating * co2_rating;
    println!("{:?}*{:?} = {:?}", oxygen_rating, co2_rating, life_support);
}

//use std::fs::File;
//use std::io::{BufRead, BufReader};

// fn part_one_arr() {
//     let mut f = BufReader::new(File::open("input.txt").unwrap());
//     let arr: Vec<Vec<f64>> = f
//         .lines()
//         .map(|l| {
//             l.unwrap()
//                 .split(char::is_whitespace)
//                 .map(|number| number.parse().unwrap())
//                 .collect()
//         })
//         .collect();

//     println!("{:?}", arr);
//     let gamma = "";
//     print_type_of(&arr);
//     // for idx in arr.iter().len(){
//     //     println!("{:?}", idx);
//     // }
// }

fn main() {
    part_one();
    part_two(&"input.txt");
}
