pub fn part1(input:&[String]) -> String {
  let (structure,moves) = parse(input);
  find_top_stacks_move_single(&mut structure.to_owned(), moves)

}
pub fn part2(input:&[String]) -> String {
  let (structure,moves) = parse(input);
  find_top_stacks_move_together(&mut structure.to_owned(), moves)
}

#[derive(Debug)]
struct Move {
  from: usize,
  to: usize,
  times: usize
}

fn find_top_stacks_move_single(structure:&mut Vec<Vec<char>>, moves:Vec<Move>) -> String {
  for m in moves {
    for _ in 0..m.times {
      let removed = structure[m.from-1].pop();
      if let Some(c) = removed {
        structure[m.to-1].push(c);
      }
    }
  }
  let mut s = String::new();
  for stack in structure {
    let ch_option = stack.pop();
    if let Some(c) = ch_option {
      s.push(c)
    }
  }
  s
}

fn find_top_stacks_move_together(structure:&mut Vec<Vec<char>>, moves:Vec<Move>) -> String {
  for m in moves {
    let from_index = structure[m.from-1].len()-m.times;
    let mut list_to_append = structure[m.from-1].drain(from_index..).collect();
    structure[m.to-1].append(&mut list_to_append);
  }
  let mut s = String::new();
  for stack in structure {
    let ch_option = stack.pop();
    if let Some(c) = ch_option {
      s.push(c)
    }
  }
  s
}

fn parse(strings:&[String]) -> (Vec<Vec<char>>,Vec<Move>) {
  let mut structure:Vec<Vec<char>> = vec![vec![];9];
  for line in strings {
    if line.is_empty() {
      break;
    }
    for (j,c) in line.chars().enumerate() {
      if c.is_alphabetic() {
        structure[(j-1)/4].push(c);
      }
    }
  }
  for stack in structure.iter_mut() {
    stack.reverse()
  }
  let mut operations = vec![];
  for line in strings {
    let splitted:Vec<&str> = line.split(' ').collect();
    if splitted[0] == "move" {
      let from = splitted[3].parse().unwrap();
      let to = splitted[5].parse().unwrap();
      let times = splitted[1].parse().unwrap();
      operations.push(Move{from,to,times})
    }
  }
  return (structure,operations)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2".lines().map(|s| String::from(s)).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!("CMZ", part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!("MCD", part2(&get_input()));
  }
}