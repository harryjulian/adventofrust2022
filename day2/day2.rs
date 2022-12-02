use std::fs;

// Get Round Score
fn map_round_outcome(p1: char, p2: char) -> i8 {
  match (p1, p2) {
    ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
    ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
    _ => 0,
  }
}

// Get Tool Score
fn map_tool_outcome(p2: char) -> i8 {
  match p2 {
    'X' => 1,
    'Y' => 2,
    'Z' => 3,
    _ => panic!()
  }
}

fn main() {

  // Read Puzzle
  let file_path = std::env::args().nth(1).expect("no path given");
  let moves = fs::read_to_string(&file_path).unwrap_or_else(|_| panic!("Error reading file {}.", file_path));

  // Raw string -> Vector!
  let moves_vectorized: Vec<&str> = moves.split('\n').collect();
  let moves_vectorized: Vec<String> = moves_vectorized.iter().map(|&s| s.into()).collect();
  println!("{moves_vectorized:?}");

  // Start the score here
  let mut overall_score: i32 = 0;
  for round in moves_vectorized.iter() {
    // Unwrap Moves
    let p1: char = round.chars().next().unwrap();
    let p2: char = round.chars().nth(2).unwrap();
    
    // Get scores
    let round_outcome = map_round_outcome(p1, p2);
    let tool_outcome = map_tool_outcome(p2);
    let round_score: i32 = (round_outcome + tool_outcome).into();

    overall_score += round_score;
  }

  println!("Solution: {overall_score:?}")

}