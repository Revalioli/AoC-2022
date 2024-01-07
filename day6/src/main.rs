use std::fs;
use std::collections::VecDeque;

fn main() {

    println!("===[ Day 6 ]===");

    match part_1() {
        Ok(res) => println!("Part 1 : {res}"),
        Err(e) => println!("Part 1 : Error ! {e}"),
    }

    match part_2() {
        Ok(res) => println!("Part 2 : {res}"),
        Err(e) => println!("Part 2 : Error ! {e}"),
    }
}


fn find_first_marker(marker_size : usize) -> Result<usize, String> {

    let mut queue : VecDeque<char> = VecDeque::with_capacity(marker_size);

    for (pos, c) in fs::read_to_string("input").unwrap().chars().enumerate() {
        if let Some(i) = queue.iter().position(|x| *x == c) {
            for _ in 0..i+1 { queue.pop_front(); };
        }
        queue.push_back(c);

        if queue.len() == marker_size { return Ok(pos+1) }
    }

    Err(format!("No marker of size {marker_size}"))
}


fn part_1() -> Result<usize, String>{ find_first_marker(4) }

fn part_2() -> Result<usize, String>{ find_first_marker(14) }


