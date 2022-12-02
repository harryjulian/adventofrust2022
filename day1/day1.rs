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

  let file_path = std::env::args().nth(1).expect("no path given");
  let elves = read(file_path);
  let solution = elves.last().unwrap().to_string();
  println!("{solution:?}")
}
