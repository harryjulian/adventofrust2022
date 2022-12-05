use std::fs;

fn main() {

  let file_path = std::env::args().nth(1).expect("no path given");
  let pairs = fs::read_to_string(&file_path).unwrap_or_else(|_| panic!("Error reading file {}.", file_path));
  let binding = pairs.replace("-", ",");
  let pairs: Vec<&str> = binding.split('\n').collect();
  
  // Get first solution
  let solution_1 = solve_1(pairs);
  println!("The solution to part 1 is: {solution_1:?}")

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