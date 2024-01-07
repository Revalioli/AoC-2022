use std::{fs, iter::repeat};

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

#[allow(dead_code)]
fn print_forest(forest : &Vec<Vec<(bool, i32)>>) {

    for l in forest {
        let line_bool : String = l.iter()
            .map(|c| if c.0 { 'X' } else { '.' })
            .collect();
        let line_heights : String = l.iter().map(|c| c.1.to_string() ).collect();
        println!("{line_bool}\t{line_heights}");
    }

}


fn part_1() -> Result<usize, String> {

    // let input = fs::read_to_string("example").unwrap();
    let input = fs::read_to_string("input").unwrap();

    // [line][col]
    let mut forest : Vec<Vec<(bool, i32)>> = input.lines()
        .map( |l| 
            l.chars()
                .map( |c| 
                    (false, c.to_digit(10)
                        .expect(&format!("Input has wrong format, encountered non digit character {c}")) as i32) 
                )
                .collect::<Vec<(bool, i32)>>()
        ).collect();
    
    // Nb lines, nb columns
    let (n, m) = (forest.len(), forest[0].len());

    let mut tallest;
    for col in 0..m {
        tallest = -1;
        // Up -> Down
        for line in 0..n {
            if forest[line][col].1 > tallest {
                tallest = forest[line][col].1;
                forest[line][col].0 = true;
            }
        }
        tallest = -1;
        // Down -> Up
        for line in (0..n).rev() {
            if forest[line][col].1 > tallest {
                tallest = forest[line][col].1;
                forest[line][col].0 = true;
            }
        }
    }
    for line in 0..n {
        tallest = -1;
        // Left -> Right
        for col in 0..m {
            if forest[line][col].1 > tallest {
                tallest = forest[line][col].1;
                forest[line][col].0 = true;
            }
        }
        tallest = -1;
        // Right -> Left
        for col in (0..m).rev() {
            if forest[line][col].1 > tallest {
                tallest = forest[line][col].1;
                forest[line][col].0 = true;
            }
        }
    }

    // print_forest(&forest);

    Ok(forest.iter().map(|l| l.iter().filter(|t| t.0).count()).sum())

}


struct Tree { height : u32, view_score : [usize; 4] }

fn scan_view<T : Iterator<Item = (usize, usize)>>(coordinates: T, forest : &mut Vec<Vec<Tree>>, score_idx : usize) {

    let mut stack : Vec<(usize, usize)> = Vec::new();
    let mut counter = 0;

    for (line, col) in coordinates {
        // Current tree is forest[line][col]

        let mut prev = stack.pop();
        while prev.is_some() {

            // Prev tree is forest[prev_line][prev_col]
            let (prev_line, prev_col) = prev.unwrap();

            if forest[prev_line][prev_col].height <= forest[line][col].height {
                // Current tree cut the view of prev_tree
                // Set the final view score for the direction and drop the mutable reference
                forest[prev_line][prev_col].view_score[score_idx] = counter - forest[prev_line][prev_col].view_score[score_idx];
            }
            else {
                // The previous tree is higher, stopping loop and putting back reference in stack
                stack.push((prev_line, prev_col));
                break;
            }

            prev = stack.pop();
        }

        // Add current tree to stack
        forest[line][col].view_score[score_idx] = counter;
        stack.push((line, col));

        counter += 1;
    }

    // Update view score of all trees in the stack
    for (line, col) in stack {
        forest[line][col].view_score[score_idx] = counter - forest[line][col].view_score[score_idx];
    }


}


fn part_2() -> Result<usize, String> {

    // let input = fs::read_to_string("example").unwrap();
    let input = fs::read_to_string("input").unwrap();

    // [line][col]
    let mut forest : Vec<Vec<Tree>> = input.lines()
        .map( |l| 
            l.chars()
                .map( |c| 
                    Tree {
                        height : c.to_digit(10).expect(&format!("Input has wrong format, encountered non digit character {c}")),
                        view_score : [0, 0, 0, 0]
                    }
                )
                .collect::<Vec<Tree>>()
        ).collect();

    // Nb lines, nb columns
    let (n, m) = (forest.len(), forest[0].len());

    // Trees on the edges have one of their view score at 0, so their overall scenic score is also 0
    // then we can just skip them in the scan

    for col in 0..m {

        let up_down = (1..n-1).zip(repeat(col));
        scan_view(up_down, &mut forest, 0);
        let down_up = (1..n-1).rev().zip(repeat(col));
        scan_view(down_up, &mut forest, 1);
    }

    for line in 0..n {

        let left_right = repeat(line).zip(1..m-1);
        scan_view(left_right, &mut forest, 2);
        let right_left = repeat(line).zip( (1..m-1).rev() );
        scan_view(right_left, &mut forest, 3);
    }


    forest.iter()
        .map(|l| 
            l.iter()
            .map(|t| t.view_score[0]*t.view_score[1]*t.view_score[2]*t.view_score[3])
            .max()
            .unwrap()
        )
        .max()
        .ok_or("No data to work on".to_string())

}