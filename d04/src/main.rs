#![allow(unused)]
use std::collections::HashMap;
use std::fs;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn part_one(filename: &str) {
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

fn main() {
    part_one(&"input.txt");
}
