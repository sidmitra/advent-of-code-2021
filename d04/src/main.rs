#![allow(unused)]
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write; // <--- bring flush() into scope

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    val: usize,
    is_marked: bool,
}

fn get_board(filename: &str) -> (Vec<usize>, Vec<Vec<Vec<Cell>>>) {
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = content.lines().collect();
    let mut lines_iter = lines.iter();

    // drawn numbers
    let drawings: Vec<usize> = lines_iter
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    // println!("{:?}", drawings);

    // Skip empty lines
    lines_iter.next().unwrap();
    // println!("=================");

    let mut current_board: Vec<Vec<Cell>> = vec![];
    let mut boards: Vec<Vec<Vec<Cell>>> = vec![];
    let num_lines = lines_iter.len();
    for (idx, line) in lines_iter.enumerate() {
        let is_separator_or_last_line = line.trim().is_empty() | (idx == num_lines - 1);
        if is_separator_or_last_line {
            // Reset board
            boards.push(current_board);
            current_board = vec![];
            // println!("====");
            continue;
        }
        let row: Vec<Cell> = line
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .map(|x| Cell {
                val: x,
                is_marked: false,
            })
            .collect();
        // println!("{:?}", row);
        current_board.push(row);
    }

    return (drawings, boards);
}

fn mark_board(drawn: usize, board: Vec<Vec<Cell>>) {
    for row in board {
        for mut cell in row {
            if cell.val == drawn {
                cell.is_marked = true;
            }
        }
    }
}


fn print_board(board: &Vec<Vec<Cell>>){
    for row in board{
        for cell in row{
            print!("{:?} ", cell.val);
            if cell.is_marked == true{
                print!("x" );
            }
        }
        println!("");
    }
    println!("===")
}


fn is_board_won(board: Vec<Vec<Cell>>) -> bool{
    // check rows
    for y in 0..5{
        // println!("{:?}", y);
        let row = &board[y];
        // rintln!("{:?}", row);
        let mut is_row_done = true;
        for cell in row {
            if !cell.is_marked{
                is_row_done = false;
            }
        }
        println!("");
    }
    return false;
}

fn part_one(filename: &str) {
    let (drawings, orig_boards) = get_board(&filename);

    let boards = orig_boards.to_owned();

    for drawn in drawings {
        println!("drawn = {:?}", drawn);
        for board in boards.to_owned() {
            // mark board
            for row in board {
                for mut cell in row.to_owned() {
                    if cell.val == drawn {
                        println!("===checked");
                        cell.is_marked = true;
                    }
                }
            }

            // check board won
            // let is_won = is_board_won(board.clone());
        }
        for board in boards.iter(){
            print_board(board);
        }
    }

    // for board in boards{
    //     for row in board{
    //         for item in row{
    //             print!("{:?} ", item.val);
    //             io::stdout().flush().unwrap();
    //         }
    //         println!{""};
    //     }
    //     println!("===============", );
    // }
    // println!("=================");
}

fn main() {
    part_one(&"input_test.txt");
}
