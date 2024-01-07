use std::fs;
use std::cmp::max;

fn main(){
    let mut ans = 0;
    let mut tot = 0;
    for i in fs::read_to_string("input").unwrap().lines() {

        if i.len() > 0 {
            tot += i.parse::<u32>().unwrap();
        }
        else{
            ans = max(tot, ans);
            tot = 0;
        }
    }

    println!("{ans}");
}