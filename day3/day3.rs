use std::fs;
use std::hash::Hash;
use std::collections::HashSet;

fn intersection<T: Eq + Hash>(a: HashSet<char>, b: &HashSet<char>) -> HashSet<char> {
  a.into_iter().filter(|e| b.contains(e)).collect()
}

fn solve_1(backpacks: Vec<&str>) {

  // Score
  let mut priority: usize = 0;

  // Define Mappings
  let alphabet: String = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
  
  let mut iter = backpacks;
  for backpack in iter {
    // Split string into compartments
    let backpack_size: usize = backpack.chars().count();
    let compartment_size: usize = backpack_size / 2;
    let a = &backpack[..compartment_size];
    let b = &backpack[compartment_size..];

    // Get compartment wise Vectors of characters
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    // Create Hashmaps
    let a_hash: HashSet<char> = a_chars.iter().cloned().collect();
    let b_hash: HashSet<char> = b_chars.iter().cloned().collect();
    
    // Get Intersections from Hashmaps
    let overlap: Vec<char> = intersection::<char>(a_hash, &b_hash).drain().collect::<Vec<char>>();
    let mut out = alphabet.find(&overlap[..1]).unwrap();

    // Add to output
    out += 1;
    priority += out;
  }
  
  println!("Solution1: {priority:?}");

}

fn main() {

  // Read file as string, convert into vector of string
  let file_path = std::env::args().nth(1).expect("no path given");
  let backpacks = fs::read_to_string(&file_path).unwrap_or_else(|_| panic!("Error reading file {}.", file_path));
  let backpacks: Vec<&str> = backpacks.split('\n').collect();

  // Solve first part
  let solution1 = solve_1(backpacks);
  
}