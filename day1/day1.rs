use std::fs;

fn read(file_path: String) -> Vec<u32> {
  let data = fs::read_to_string(&file_path).unwrap_or_else(|_| panic!("Error reading file {}", file_path));
  let mut out = data
      .replace('\r', "")
      .split("\n\n")
      .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum())
      .collect::<Vec<_>>();
  out.sort();
  out
}

fn main() {

  // Parse .txt file
  let file_path = std::env::args().nth(1).expect("no path given");
  let elves = read(file_path);
  
  // Get Solution 1
  let solution1 = elves.last().unwrap().to_string();

  // Get Solution 2
  let last3 = elves.as_slice()[elves.len()-3..].to_vec();
  let solution2: u32 = last3.iter().map(|&i| i as u32).sum();

  // Print Solutions
  println!("Solution 1: {solution1:?}");
  println!("Solution 2: {solution2:?}");
}
