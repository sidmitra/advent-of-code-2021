use std::fs;

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

    println!("{:?}", increased_count);
}
