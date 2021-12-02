
use std::fs;

// Part 2: 1749 is too high

fn main() {
    let filename = "input.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let numbers: Vec<i32> = content.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut increased_count: i16 = 0;
    let mut last_num: &i32 = &numbers[0];

    for (idx, num) in numbers.iter().enumerate() {
        // println!("{:?}", num);
        if idx == 0 {
            continue;
        }
        if num > last_num {
            // println!("increased");
            increased_count += 1;
        }
        last_num = num;
    }

    // println!("Part 1: {:?}", increased_count);


    // Part 2
    let mut increased_count_two: i32 = 0;
    let mut last_sum: i32 = 0;

    // println!("window          | last_sum | current_sum");
    for (idx, window) in numbers.windows(3).enumerate(){
        // Skip comparison in the first iteration
        if idx == 0{
            last_sum = window.iter().sum();
            continue
        }

        let current_sum: i32 = window.iter().sum();
        // println!("{:?} | {:?}      | {:?}", window, last_sum, current_sum);

        if current_sum > last_sum{
            increased_count_two += 1;
        }
        last_sum = current_sum;
    }

    println!("Part 2: {:?}", increased_count_two);
}
