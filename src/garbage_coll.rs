use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

use crate::types::{Memory, RefCountMem};


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn reference_counting(filename: &str) ->  RefCountMem {
    // given code iterates through the lines in the file; you may mpdify it if desired
 
    todo!(); // Define any variables to store the data as you go

    let heap_ref_re = Regex::new(r"Ref Heap (([0-9]+) ?)*").unwrap();
    let stack_ref_re = Regex::new(r"Ref Stack (([0-9]+) ?)*").unwrap();
    let num_lst_re = Regex::new(r"([0-9]+) ?").unwrap();
    let pop_re = Regex::new(r"Pop").unwrap();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            if heap_ref_re.is_match(&line) {
                let mut numbers: Vec<u32> = vec![]; // vec of the numbers in the line
                num_lst_re.captures_iter(&line).for_each(|f | {
                    numbers.push(f.get(1).unwrap().as_str().parse::<u32>().unwrap());
                }); // i.e. Ref Heap 0 1 2 -> numbers is now: [0, 1, 2]. 
                // numbers is given to you and will correctly represent the file, you just need to handle the garbage collection now

                todo!() // handle case of Ref Heap here

            } else if stack_ref_re.is_match(&line) {
                let mut numbers: Vec<u32> = vec![]; // vec of the numbers in the line

                num_lst_re.captures_iter(&line).for_each(|f | {
                    numbers.push(f.get(1).unwrap().as_str().parse::<u32>().unwrap());
                }); // i.e. Ref Stack 0 1 2 -> numbers is now: [0, 1, 2]
                // numbers is given to you and will correctly represent the file, you just need to handle the garbage collection now

                todo!() // handle case of Ref Stack here

            } else if pop_re.is_match(&line) {
                
                todo!() // handle Pop case here

            } else {
                panic!("no matches");
            }
        }
    };

    // If you haven't already, free all memory that has 0 references here
    // then, return the RefCountMem struct
    todo!()

}

// suggested helper function. You may modify parameters as you wish.
// Takes in some form of stack and heap and returns all indicies in heap
// that can be reached.
pub fn reachable(stack: &Vec<Vec<u32>>, heap: &Vec<Option<(String, Vec<u32>)>>) -> Vec<u32> {
    todo!()
} 

pub fn mark_and_sweep(mem: &mut Memory) -> () {
    todo!()
}
// alive says which half is CURRENTLY alive. You must copy to the other half
// 0 for left side currently in use, 1 for right side currently in use
pub fn stop_and_copy(mem: &mut Memory, alive: u32) -> () {
    todo!()
}