use std::fs;
use std::ops::Range;

fn main() {

    println!("===[ Day 4 ]===");

    match part_1() {
        Ok(res) => println!("Part 1 : {res}"),
        Err(e) => println!("Part 1 : Error ! {e}"),
    }

    match part_2() {
        Ok(res) => println!("Part 2 : {res}"),
        Err(e) => println!("Part 2 : Error ! {e}"),
    }
}



trait ContainsRange<Idx> 
{
    fn contains_range(&self, r : &Range<Idx>) -> bool;
    fn overlap_range(&self, r : &Range<Idx>) -> bool;
}

impl ContainsRange<i32> for Range<i32>
{

    fn contains_range(&self, r : &Range<i32>) -> bool {
        self.start <= r.start && self.end >= r.end
    }

    fn overlap_range(&self, r : &Range<i32>) -> bool {
        self.contains(&r.start) || self.contains(&(r.end-1)) || r.contains(&self.start) || r.contains(&(self.end-1))
    }

}



fn part_1() -> Result<u32, &'static str> {

    let file_str = fs::read_to_string("input").expect("Error while reading input file");
    let mut score = 0;

    for line in file_str.lines() {

        let pair : Vec<&str> = line.split(',').collect();

        let mut str_range = pair[0].split("-");
        let mut left_range : Range<i32> = str_range.next().unwrap().parse().unwrap()..(str_range.next().unwrap().parse().unwrap());
        left_range.end += 1;
        let mut str_range = pair[1].split("-");
        let mut right_range : Range<i32> = str_range.next().unwrap().parse().unwrap()..(str_range.next().unwrap().parse().unwrap());
        right_range.end += 1;

        if left_range.contains_range(&right_range) || right_range.contains_range(&left_range) {
            score += 1;
        }

    }

    Ok(score)
}

fn part_2() -> Result<u32, &'static str> {
    let file_str = fs::read_to_string("input").expect("Error while reading input file");
    let mut score = 0;

    for line in file_str.lines() {

        let pair : Vec<&str> = line.split(',').collect();

        let mut str_range = pair[0].split("-");
        let mut left_range : Range<i32> = str_range.next().unwrap().parse().unwrap()..(str_range.next().unwrap().parse().unwrap());
        left_range.end += 1;
        let mut str_range = pair[1].split("-");
        let mut right_range : Range<i32> = str_range.next().unwrap().parse().unwrap()..(str_range.next().unwrap().parse().unwrap());
        right_range.end += 1;

        if left_range.overlap_range(&right_range) {
            score += 1;
        }

    }

    Ok(score)
}





#[cfg(test)]
mod tests {
    use crate::ContainsRange;


    #[test]
    fn contains_range_trait() {
        let reference = -5..9;

        assert!(reference.contains_range(&(-4..8)));
        assert!(reference.contains_range(&(-5..9)));
        assert!(!reference.contains_range(&(6..10)));
        assert!(!reference.contains_range(&(-320..-54)));
        assert!(!reference.contains_range(&(-6..11)));

    }

}

