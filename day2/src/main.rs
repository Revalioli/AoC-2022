use std::fs;

fn main() {

    println!("===[ Day 2 ]===");

    match part_1() {
        Ok(res) => println!("Part 1 : {res}"),
        Err(e) => println!("Part 1 : Error ! {e:?}"),
    }

    match part_2() {
        Ok(res) => println!("Part 2 : {res}"),
        Err(e) => println!("Part 2 : Error ! {e:?}"),
    }
}

fn part_1() -> Result<i32, String> {
    
    let mut score = 0;

    for i in fs::read_to_string("input").expect("Error while reading input file").lines() {
        let mut c = i.chars();
        let opponent = c.next().unwrap();
        let to_play = c.nth(1).unwrap();

        score += match to_play {
            'X' => {    // Rock
                match opponent {
                    'A' => 3 + 1,   // Rock -> Draw
                    'B' => 1,       // Paper -> Defeat
                    'C' => 6 + 1,   // Scissors -> Win
                    other => return Err(format!("Can't handle character {other}"))
                }
            }
            'Y' => {    // Paper
                match opponent {
                    'A' => 6 + 2,   // Rock -> Win
                    'B' => 3 + 2,   // Paper -> Draw
                    'C' => 2,       // Scissors -> Defeat
                    other => return Err(format!("Can't handle character {other}"))
                }
            }
            'Z' => {    // Scissors
                match opponent {
                    'A' => 3,       // Rock -> Defeat
                    'B' => 6 + 3,   // Paper -> Win
                    'C' => 3 + 3,   // Scissors -> Draw
                    other => return Err(format!("Can't handle character {other}"))
                }
            }
            other => return Err(format!("Can't handle character {other}")),
        }

    }
    
    Ok(score)
    
}

fn part_2() -> Result<i32, String> {

    let mut score = 0;

    for i in fs::read_to_string("input").expect("Error while reading input file").lines() {
        let mut c = i.chars();
        let opponent = c.next().unwrap();
        let to_play = c.nth(1).unwrap();

        score += match to_play {
            'X' => {    // Need to lose
                match opponent {
                    'A' => 3,   // Rock -> must play scissors
                    'B' => 1,   // Paper -> must play rock
                    'C' => 2,   // Scissors -> must play paper
                    other => return Err(format!("Can't handle character {other}"))
                }
            }
            'Y' => {    // Need to draw
                3 + match opponent {
                    'A' => 1,   // Rock -> must play rock
                    'B' => 2,   // Paper -> must play paper
                    'C' => 3,   // Scissors -> must play scissors
                    other => return Err(format!("Can't handle character {other}"))
                }
            }
            'Z' => {    // Need to win
                6 + match opponent {
                    'A' => 2,       // Rock -> must play paper
                    'B' => 3,   // Paper -> must play scissors
                    'C' => 1,   // Scissors -> must play rock
                    other => return Err(format!("Can't handle character {other}"))
                }
            }
            other => return Err(format!("Can't handle character {other}")),
        }

    }
    
    Ok(score)

}
