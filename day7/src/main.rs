use std::fs;
use std::str::Lines;

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

#[derive(Debug)]
enum Node {
    Dir(Directory),
    File(u32)
}

#[derive(Debug)]
struct Directory {
    name : String,
    children : Vec<Node>,
    size : u32
}


impl Directory {

    fn from_input(input : &str) -> Self {

        let mut iter = input.lines();

        let mut root = Self { name : String::from("root"), children : Vec::new(), size : 0 };
        root.build_directory(&mut iter, true);

        root

    }

    fn compute_size(&mut self) {
        self.size = self.children
                        .iter()
                        .fold(0, |acc, x|
                            match x {
                                Node::File(s) => acc + s,
                                Node::Dir(d) => acc + d.size
                            }
                        )
    }

    fn build_directory(&mut self, input : &mut Lines<'_>, is_root : bool) -> &'static str {

        while let Some(l) = input.next() {

            let mut spliter = l.split_whitespace();

            match spliter.next().unwrap() {
                "$" => {
                    // Command
                    match spliter.next().unwrap() {
                        "cd" => {
                            // Where to go
                            match spliter.next().unwrap() {
                                "/" if !is_root => {
                                    self.compute_size();
                                    return "/";
                                }
                                "/" => {},
                                ".." => {
                                    self.compute_size();
                                    return "..";
                                }
                                sub_dir => {

                                    if let "/" = self.children.iter_mut()
                                        .find_map(|x|       // Find the child with the corresponding name
                                            if let Node::Dir(d) = x {
                                                if d.name == sub_dir { Some(d) } else { None }
                                            }
                                            else { None }
                                        ).unwrap()      // Get the mutable reference
                                        .build_directory(input, false)     // Get in it to create the subdirectory
                                    {
                                        if !is_root {
                                            // In case we need to go back to the root
                                            self.compute_size();
                                            return "/";
                                        }
                                    }

                                }
                            }
                        },
                        "ls" => {},
                        unknown_cmd => panic!("Unexpected command {unknown_cmd}")
                    }

                },
                "dir" => {
                    // Create new sub directory
                    self.children.push( Node::Dir( Directory{ name : spliter.next().unwrap().to_string(), children : Vec::new(), size : 0 } ) );
                },
                size => {
                    self.children.push( Node::File(size.parse::<u32>().expect("Expected file size, line has unexpected format")) );
                }
            }

        }
        self.compute_size();
        "/"

    }

    // For part 1
    fn sum_small_dir(&self, limit : u32) -> u32 {
        self.children.iter()
            .fold(0, |acc, c|
                match c {
                    Node::Dir(d) => acc + d.sum_small_dir(limit),
                    _ => acc
                }
            ) + if self.size <= limit { self.size } else { 0 }
    }

    // For part 2
    fn best_deletion(&self, required_space : u32) -> Option<&Directory> {

        let best_child = self.children
            .iter()
            .fold( Option::<&Directory>::None, |acc, n|
                match n {
                    Node::Dir(d) => {
                        let best = d.best_deletion(required_space);
                        match (acc, best) {
                            ( Some(acc_d), Some(d) ) if d.size < acc_d.size && d.size >= required_space => best,
                            ( None, Some(_) ) => best,
                            _ => acc
                        }
                    }
                    _ => acc
                }
            );
        

        // Checking if current dir is more suitable to deletion, and return better choice
        if self.size >= required_space {
            match best_child {
                Some(d) if self.size < d.size => Some(self),    // Current dir is a better choice
                Some(_) => best_child,  // Current dir is not better that best child
                None => Some(self)      // Current dir is better than nothing
            }
        }
        else { best_child } // Current dir is too small anyway

    }

}



fn part_1() -> Result<u32, String>{

    let input = fs::read_to_string("input").unwrap();
    // let input = fs::read_to_string("example").unwrap();
    let root = Directory::from_input(&input);

    println!("{root:#?}");

    Ok(root.sum_small_dir(100000))
}

fn part_2() -> Result<u32, String>{

    let input = fs::read_to_string("input").unwrap();
    // let input = fs::read_to_string("example").unwrap();
    let root = Directory::from_input(&input);

    let required_space = 30000000 - (70000000 - root.size);

    root.best_deletion(required_space).map(|x| x.size ).ok_or("No solution found".to_string())
}
