use std::fs;
use std::collections::HashSet;
use std::convert::TryInto;

fn main() {

  let file_path = std::env::args().nth(1).expect("no path given");
  let code = fs::read_to_string(&file_path).unwrap_or_else(|_| panic!("Error reading file {}.", file_path));
  let code: Vec<char> = code.chars().collect();
  let full_len: i32 = code.len().try_into().unwrap();

  let solution_1 = solve(code.clone(), 4);
  let solution_2 = solve(code, 14);
  println!("Solution to 1 is: {solution_1:?}");
  println!("Solution to 2 is: {solution_2:?}");
}

fn unique(block: &[char], window_size: &i32) -> bool {
  let out: bool = false;

  let dedup_hash: HashSet<char> = block.iter().cloned().collect();
  let full_len: i32 = *window_size;
  let unique_len: i32 = dedup_hash.len().try_into().unwrap();

  if unique_len == full_len {
    println!("{dedup_hash:?}");
    let out = true;
    return out
  }
  out
}

fn solve(code: Vec<char>, window_size: i32) -> i32 {
  let mut out: i32 = 0; 
    
    for block in code.windows(window_size.try_into().unwrap()) {
      out += 1;
      if unique(block, &window_size) {
        return out + (window_size - 1);
      }
    }
  out
}

