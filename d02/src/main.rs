use std::fs;
// use itertools::Itertools; // Didn't want to install crate

// --- Part Two ---

// Based on your calculations, the planned course doesn't seem to make any sense. You find the submarine manual and discover that the process is actually slightly more complicated.

// In addition to horizontal position and depth, you'll also need to track a third value, aim, which also starts at 0. The commands also mean something entirely different than you first thought:

//     down X increases your aim by X units.
//     up X decreases your aim by X units.
//     forward X does two things:
//         It increases your horizontal position by X units.
//         It increases your depth by your aim multiplied by X.

// Again note that since you're on a submarine, down and up do the opposite of what you might expect: "down" means aiming in the positive direction.

// Now, the above example does something different:

//     forward 5 adds 5 to your horizontal position, a total of 5. Because your aim is 0, your depth does not change.
//     down 5 adds 5 to your aim, resulting in a value of 5.
//     forward 8 adds 8 to your horizontal position, a total of 13. Because your aim is 5, your depth increases by 8*5=40.
//     up 3 decreases your aim by 3, resulting in a value of 2.
//     down 8 adds 8 to your aim, resulting in a value of 10.
//     forward 2 adds 2 to your horizontal position, a total of 15. Because your aim is 10, your depth increases by 2*10=20 to a total of 60.

// After following these new instructions, you would have a horizontal position of 15 and a depth of 60. (Multiplying these produces 900.)

// Using this new interpretation of the commands, calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn main() {
    let filename = "input.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let rows: Vec<&str> = content.lines().collect();
    for row in rows {
        // let mut words = row.split(" ").collect::<Vec<&str>>();
        // let (direction, distance_str) = row.splitn(2, " ").collect_tuple().unwrap();
        let mut words = row.splitn(2, " ");
        let direction = words.next().unwrap();
        let distance_str = words.next().unwrap(); // .to_string().parse::<i32>.unwrap();
                                                  // print_type_of(&distance_str);
        let distance = distance_str.parse::<i32>().unwrap();
        // println!("{:?} {:?}", direction, distance);

        match direction {
            "forward" => x = x + distance,
            "down" => y = y + distance,
            "up" => y = y - distance,
            _ => println!("Invalid direction"),
        }
    }

    println!("Part 1: {:?} * {:?}={:?}", x, y, x * y);

    // Part 2

    x = 0;
    y = 0;
    let mut aim: i32 = 0;

    let rows: Vec<&str> = content.lines().collect();
    for row in rows {
        let mut words = row.splitn(2, " ");
        let direction = words.next().unwrap();
        let distance_str = words.next().unwrap();
        let distance = distance_str.parse::<i32>().unwrap();

        match direction {
            "forward" =>
            {
                x = x + distance;
                y = y + aim * distance;
            } ,
            "down" => {
                //y = y + distance;
                aim = aim + distance;
            },
            "up" => {
                aim = aim - distance;
            },
            _ => println!("Invalid direction"),
        }
    }

    println!("Part 2: {:?} * {:?}={:?}", x, y, x * y);
}
