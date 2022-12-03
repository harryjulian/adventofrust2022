use std::fs;
use std::hash::Hash;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

fn intersection<T: Eq + Hash>(a: HashSet<char>, b: &HashSet<char>) -> HashSet<char> {
  a.into_iter().filter(|e| b.contains(e)).collect()
}

fn solve_1(backpacks: Vec<&str>) {

  // Score
  let mut priority: i16 = 0;

  // Define Mappings
  let alphabet_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
  let alphabet = alphabet_chars.collect::<Vec<_>>();
  let numbers: Vec::<i16> = (1..=52).map(|x| x as i16).collect::<Vec<i16>>();

  // Get mappings into Hashmap
  let mut map_iter = alphabet.iter().zip(numbers.iter());
  let mapping: HashMap<&char, &i16> = HashMap::from_iter(map_iter);

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
    let out = mapping.get(&overlap[..1]);

    // Add to overall
    priority += out
  }
  
  println!("Solution1: {priority:?}");

}

fn main() {

  // Read file as string, convert into vector of string
  let file_path = std::env::args().nth(1).expect("no path given");
  let backpacks = fs::read_to_string(&file_path).unwrap_or_else(|_| panic!("Error reading file {}.", file_path));
  let backpacks: Vec<&str> = backpacks.split('\n').collect();

  let solution1 = solve_1(backpacks);
  
}