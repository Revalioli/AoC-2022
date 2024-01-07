use std::fs;

fn main() {
    let mut ans = [0, 0, 0];
    let mut tot = 0;
    for i in fs::read_to_string("input").unwrap().lines(){

        if i.len() > 0 {
            tot += i.parse::<u32>().unwrap();
        }
        else{
            let (i, val) : (usize, &u32) = ans.iter().enumerate().min_by_key(|(_, &v)| v).unwrap();

            if val < &tot {
                ans[i] = tot;
            }
            tot = 0;
        }

    }

    println!("{}", ans.iter().sum::<u32>());
}