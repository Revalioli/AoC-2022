use std::fs;
use std::collections::HashSet;

fn main() {

    println!("===[ Day 3 ]===");

    match part_1() {
        Ok(res) => println!("Part 1 : {res}"),
        Err(e) => println!("Part 1 : Error ! {e:?}"),
    }

    match part_2() {
        Ok(res) => println!("Part 2 : {res}"),
        Err(e) => println!("Part 2 : Error ! {e:?}"),
    }
}


fn get_priority(c : char) -> u8 {

    if c.is_lowercase() {
        return (c as u8) - 96; 
    }
    else if c.is_uppercase() {
        return (c as u8) - 38;
    }
    
    panic!("{c} is not an ASCII leter character.");

}


fn part_1() -> Result<u32, String> {
    
    let mut score: u32 = 0;

    for i in fs::read_to_string("input").expect("Error while reading input file").lines() {

        let h_len = i.len()/2;
        let left = &i[..h_len];
        let right = &i[h_len..];

        let mut in_left: HashSet<char> = HashSet::new();

        for c in left.chars() {
            in_left.insert(c);
        }

        for c in right.chars() {
            if in_left.contains(&c) {

                score += get_priority(c) as u32;
                
                break;
            }
        }

    }

    Ok(score)

}


#[derive(Debug)]
struct Group<'a> {
    elf1 : &'a str,
    elf2 : &'a str,
    elf3 : &'a str
}

impl<'a> Group<'a> {

    fn new() -> Self {
        Self {
           elf1 : "",
           elf2 : "",
           elf3 : ""
        }
    }

    fn fetch_elves(&mut self, lines : &mut std::str::Lines<'a>) -> std::io::Result<()> {
        
        
        let temp = lines.next();

        if temp.is_none() {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }

        self.elf1 = temp.unwrap();

        self.elf2 = lines.next().unwrap();
        self.elf3 = lines.next().unwrap();
        Ok(())
    }

}

fn part_2() -> Result<u32, String> {

    let mut score = 0;
    let mut g = Group::new();

    let file_str = fs::read_to_string("input").expect("Error while reading input file");
    let mut file_it = file_str.lines();

    'global : loop {

        if let Err(_) = g.fetch_elves(&mut file_it) {
            // return Err(format!("{} : no data to be read", err.to_string()));
            break;
        }

        let mut candidates: HashSet<char> = HashSet::new();

        for c in g.elf1.chars() {
            candidates.insert(c);
        }


        let mut new_candidates : HashSet<char> = HashSet::new();
        for c in g.elf2.chars() {
            if candidates.contains(&c) {
                new_candidates.insert(c);
            }
        }

        for c in g.elf3.chars() {
            if new_candidates.contains(&c) {
                score += get_priority(c) as u32;
                continue 'global;
            }
        }

        return Err(format!("No common character for group {g:?}"));
    

    }

    
    Ok(score)
}
