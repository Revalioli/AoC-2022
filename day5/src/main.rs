use std::fs;

fn main() {

    println!("===[ Day 5 ]===");

    match part_1() {
        Ok(res) => println!("Part 1 : {res}"),
        Err(e) => println!("Part 1 : Error ! {e}"),
    }

    match part_2() {
        Ok(res) => println!("Part 2 : {res}"),
        Err(e) => println!("Part 2 : Error ! {e}"),
    }
}


pub mod warehouse {

    use std::str;

	#[derive(Clone, Copy)]
	pub enum CrateMover {
		T9000,	// Part 1
		T9001	// Part 2
	}


    pub struct Move{ nb_crates : usize, src : usize, dst : usize }

    #[derive(Debug)]
    pub struct SupplyStock<'a> {
        pub stacks : Vec<Vec<char>>,
        instructions_iter : str::Lines<'a>
    }

    impl<'a> SupplyStock<'a> {
        
        /// Creates a SupplyStock from input data given as a string.
        /// This string is typically the content of the input for the day, and must follow the same format.
        /// The returned instance must outlived inpout_file_name.
        pub fn from_input(input_file_name : &'a str) -> Self {

            let mut file_lines = input_file_name.lines();
            let mut stacks : Vec<Vec<char>> = Vec::new();

            let mut init = true;

            loop {

                let line = file_lines.next().expect("Input has wrong format, can't find end of stacks data");

                // Create all Vec
                if init {
                    let nb_stacks = (line.chars().count() + 1) / 4;
                    stacks = vec![Vec::new(); nb_stacks];
                    init = false;
                }

                // End parsing condition
                if line.starts_with(" 1") {
                    break;
                }

                let mut enum_chars = line.chars().enumerate();
                let mut current = enum_chars.nth(1);

                // Parsing line
                while let Some( (i, c) ) = current {

                    if c.is_ascii_alphabetic() {
                        stacks[i/4].push(c);
                    }

                    current = enum_chars.nth(3);
                }

            }

            file_lines.next();

            // Reversing every vector to put the lowest crate at index 0
            for sub_v in &mut stacks {
                sub_v.reverse();
            }

            Self {
                stacks : stacks,
                instructions_iter : file_lines
            }

        }

        /// Parse the next instruction line, consuming it, and return a Move object.
        /// If there is no next instruction (self.instructions_iter has been completly consumed), return `None`.
        fn parse_next_instruction(&mut self) -> Option<Move> {
            if let Some(line) = self.instructions_iter.next() {

                let mut line = line.split_whitespace();

                return Some(Move{
                    nb_crates : line.nth(1).unwrap().parse().unwrap(),
                    src : line.nth(1).unwrap().parse().unwrap(),
                    dst : line.nth(1).unwrap().parse().unwrap()
                })

            }
            
            None
        }

        /// Execute the next instruction, returning Some(Move) for the current instruction.
        /// If there is no instruction left, return None.
        pub fn next_instruction(&mut self, mover : CrateMover) -> Option<Move> {
            let m = self.parse_next_instruction()?;

			match mover {
				CrateMover::T9000 => self.move_with_9000(m),
				CrateMover::T9001 => self.move_with_9001(m)
			}
        }


		fn move_with_9000(&mut self, m : Move) -> Option<Move> {
			for _ in 0..m.nb_crates {
				let c = self.stacks[m.src-1].pop().unwrap();
				self.stacks[m.dst-1].push(c);
			}

			Some(m)
		}

		fn move_with_9001(&mut self, m : Move) -> Option<Move> {
			
			let mut buff : Vec<char> = Vec::new();
			
			for _ in 0..m.nb_crates {
				buff.push(self.stacks[m.src-1].pop().unwrap());
			}

			self.stacks[m.dst-1].extend(buff.iter().rev());

			Some(m)
		}


		
		/// Execute all remaining move instructions.
		pub fn all_instructions(&mut self, mover : CrateMover) {
			while let Some(_) = self.next_instruction(mover) {}
        }


        pub fn get_top_crates(&self) -> String {
			self.stacks.iter().map(|v| v.last().unwrap()).collect()
        }

    }

}


use warehouse::*;

fn part_1() -> Result<String, &'static str>{

    let input = fs::read_to_string("input").expect("Error while reading input file");
	// let input = fs::read_to_string("example").expect("Error while reading input file");
    let mut supply = SupplyStock::from_input(&input);

	supply.all_instructions(CrateMover::T9000);

    Ok(supply.get_top_crates())
}

fn part_2() -> Result<String, &'static str>{
    let input = fs::read_to_string("input").expect("Error while reading input file");
	// let input = fs::read_to_string("example").expect("Error while reading input file");
    let mut supply = SupplyStock::from_input(&input);

	supply.all_instructions(CrateMover::T9001);

    Ok(supply.get_top_crates())
}
