pub fn part1(input:&[String]) -> usize {
  let mut score = 0;
  for l in input {
    score += play(parse(l))
  }   
  score
}
pub fn part2(input:&[String]) -> usize {
  let mut score = 0;
  for l in input {
    score += play_2(parse(l))
  }   
  score
}

fn parse(string: &String) -> (usize,usize) {
  let values:Vec<usize> = string.split(' ').map(|c| convert_move(c)).collect();
  (values[0],values[1])
}
// Rock= A,X,1 Paper = B,Y,2 Scissors C,Z,2
fn convert_move(s: &str) -> usize{
  match s {
     "A"|"X" => 1,
     "B"|"Y" => 2,
     "C"|"Z" => 3,
     &_ => 0
  }
}
// 0 lost, 3 draw, 6 win
fn play((move1,move2):(usize,usize)) -> usize {
  if move1==move2 {
    3+move2
  } else {
    match (move1,move2) {
      (1,2) => 6+move2,
      (1,3) => 0+move2,
      (2,1) => 0+move2,
      (2,3) => 6+move2,
      (3,1) => 6+move2,
      (3,2) => 0+move2,
      _ => 0
    }
  }
}
// Rock= A,X,1 Paper = B,Y,2 Scissors C,Z,3
fn play_2((mov,outcome):(usize,usize)) -> usize {
  if outcome == 2 {
    3 + mov
  } else {
    match (mov,outcome) {
      (1,1) => 0+3,
      (1,3) => 6+2,
      (2,1) => 0+1,
      (2,3) => 6+3,
      (3,1) => 0+2,
      (3,3) => 6+1,
      _ => 0
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "A Y
    B X
    C Z".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(15, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(12, part2(&get_input()));
  }
}