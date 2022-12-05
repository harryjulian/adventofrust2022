use std::fs;
use std::collections::HashSet;
use std::convert::TryInto;

fn main() {

  let file_path = std::env::args().nth(1).expect("no path given");
  let pairs = fs::read_to_string(&file_path).unwrap_or_else(|_| panic!("Error reading file {}.", file_path));
  let binding = pairs.replace("-", ",");
  let pairs: Vec<&str> = binding.split('\n').collect();
  
  // Get first solution
  let solution_1 = solve_1(pairs.clone());
  println!("The solution to part 1 is: {solution_1:?}");

  // Get second solution
  let solution_2 = solve_2(pairs.clone());
  println!("The solution to part 2 is: {solution_2:?}");

}

fn solve_1(pairs: Vec<&str>) -> i32 {

  // In how many assignment pairs does one range fully contain the other?
  let mut solution: i32 = 0;
  let iter = pairs;

  for pair in iter {
    let pair_vec: Vec<i32> = pair.split(",").map(|x| x.parse().unwrap()).collect();
    let a = pair_vec[0];
    let b = pair_vec[1];
    let c = pair_vec[2];
    let d = pair_vec[3];
    
    if (a >= c) && (b <= d) {
      solution += 1;
    } else if (a <= c) && (b >= d) {
      solution +=1;
    }
  
  }
  solution
}

fn solve_2(pairs: Vec<&str>) -> i32 {

  // In how many assignment pairs do the ranges overlap?
  let mut solution: i32 = 0;
  let iter = pairs;

  for pair in iter {
    let pair_vec: Vec<i32> = pair.split(",").map(|x| x.parse().unwrap()).collect();
    let first_range = (pair_vec[0]..pair_vec[1]).collect::<HashSet<_>>();
    let second_range = (pair_vec[2]..pair_vec[3]).collect::<HashSet<_>>();
    let intersect = first_range.intersection(&second_range).collect::<Vec<_>>();
    let overlap: i32 = intersect.len().try_into().unwrap();
    if overlap >= 1 {
      solution += 1;
    } 
  }
  solution
}