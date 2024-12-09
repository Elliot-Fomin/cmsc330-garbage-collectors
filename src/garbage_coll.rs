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
 
    let mut s: Vec<Vec<u32>> = vec![]; 
    let mut h: Vec<(Option<Vec<u32>>, u32)> = vec![(None, 0); 10];


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

                let index = numbers.remove(0);
                h[index as usize].0 = Some(numbers.clone());
                for number in numbers{
                    h[number as usize].1 = h[number as usize].1 + 1;
                    if h[number as usize].0 == None{
                        h[number as usize].0 = Some(vec![]);
                    }
                }




            } else if stack_ref_re.is_match(&line) {
                let mut numbers: Vec<u32> = vec![]; // vec of the numbers in the line

                num_lst_re.captures_iter(&line).for_each(|f | {
                    numbers.push(f.get(1).unwrap().as_str().parse::<u32>().unwrap());
                }); // i.e. Ref Stack 0 1 2 -> numbers is now: [0, 1, 2]
                // numbers is given to you and will correctly represent the file, you just need to handle the garbage collection now

                s.push(numbers.clone());
                for number in numbers{
                    h[number as usize].1 += 1;
                    if h[number as usize].0 == None{
                        h[number as usize].0 = Some(vec![]);
                    }
                }

            } else if pop_re.is_match(&line) {
                
                let mut numbers = s.remove(s.len()-1);
                let mut i = 0;
                while i < numbers.len(){
                    let index = *numbers.get(i).unwrap();
                    h[index as usize].1 -= 1;
                    if h[index as usize].1 == 0{
                        numbers.extend(h[index as usize].0.clone().unwrap())
                    }
                    i+=1;
                }

            } else {
                panic!("no matches");
            }
        }
    };

    for index in &mut h{
        if index.1 == 0{
            index.0 = None;
        }
    }

    let output = RefCountMem {
        stack: s,
        heap: h,
    };

    output

}

// suggested helper function. You may modify parameters as you wish.
// Takes in some form of stack and heap and returns all indicies in heap
// that can be reached.
pub fn reachable(stack: &Vec<Vec<u32>>, heap: &Vec<Option<(String, Vec<u32>)>>) -> Vec<u32> {
    let mut canReach: Vec<u32> = vec![];
    for lst in stack{
        for index in lst{
            if !canReach.contains(&index){
                canReach.push(*index);
            }
        }   
    }
    let mut index = 0;
    while index < canReach.len(){
        let i = canReach.get(index).unwrap();
        for j in heap[*i as usize].clone().unwrap().1{
            if !canReach.contains(&j){
                canReach.push(j);
            }
        }
        index +=1;
    }
    canReach
} 

pub fn mark_and_sweep(mem: &mut Memory) -> () {
    let reachable: Vec<u32> = reachable(&mem.stack,&mem.heap);
    let mut i =0;
    while i < mem.heap.len(){
        if !reachable.contains(&(i as u32)){
            mem.heap[i] = None;
        }
        i+=1;
    }
}
// alive says which half is CURRENTLY alive. You must copy to the other half
// 0 for left side currently in use, 1 for right side currently in use

pub fn stop_and_copy(mem: &mut Memory, alive: u32) -> () {

    let mut startAlive = alive * (mem.heap.len()/2) as u32;
    let mut startDead = (1-alive)*(mem.heap.len()/2) as u32;
    let reachable: Vec<u32> = reachable(&mem.stack,&mem.heap);
    let mut newPos = HashMap::new();
    while startAlive < mem.heap.len() as u32{
        if reachable.contains(&startAlive){
            mem.heap[startDead as usize] = mem.heap[startAlive as usize].clone();
            newPos.insert(startAlive, startDead);
            let keys: Vec<_> = newPos.keys().cloned().collect();
    
            println!("{:?}", keys); 

            startDead +=1;
        }
        startAlive += 1;
    }
    while startDead < ((1-alive) * (mem.heap.len()/2) as u32) + (mem.heap.len()/2) as u32{
        mem.heap[startDead as usize] = None;
        startDead +=1;
    }
    let mut index = (1-alive)*(mem.heap.len()/2) as u32;
    let fin = index + (mem.heap.len()/2) as u32;
    while index < fin{
        if let Some((name, vec)) = mem.heap[index as usize].clone(){
            let mut newVec :Vec<u32> = vec![];
            for i in vec{
                newVec.push(*newPos.get(&i).unwrap());
            }
            mem.heap[index as usize] = Some((name, newVec));
        }
        index +=1;
    }
    let mut index = 0;
    while index < mem.stack.len(){
        let mut newVec :Vec<u32> = vec![];
        for i in mem.stack[index as usize].clone(){
            println!("{} stack",i);
            newVec.push(*newPos.get(&i).unwrap());
        }
        mem.stack[index as usize] = newVec;
        index +=1;
    }

}