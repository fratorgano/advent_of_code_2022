use std::{collections::{HashSet}, iter::FromIterator, vec};

pub fn part1(input:&[String]) -> usize {
  // let parsed = parse(input);
  let mut total= 0;
  for l in input {
    let chars:Vec<char> = l.chars().collect();
    let first_half = &chars[0..chars.len()/2];
    let second_half = &chars[chars.len()/2..chars.len()];
    let first_set = HashSet::<&char>::from_iter(first_half.iter());
    let second_set = HashSet::<&char>::from_iter(second_half.iter());
    let res:usize = second_set.intersection(&first_set).map(|c| char_to_priority(**c)).next().unwrap();
    total += res
  }
  total
}
pub fn part2(input:&[String]) -> usize {
  // let parsed = parse(input);
  let mut total = 0;
  let groups:Vec<&[String]> = input.chunks(3).collect();
  for group in groups {
    let mut hash_sets = vec![];
    for elem in group {
      let set = HashSet::<char>::from_iter(elem.chars());
      hash_sets.push(set)
    }
    let inter1:HashSet<_> = hash_sets[0].intersection(&hash_sets[1]).collect();
    let inter2:HashSet<_> = hash_sets[1].intersection(&hash_sets[2]).collect();
    let inter = inter1.intersection(&inter2).next().unwrap();
    total += char_to_priority(**inter);
  }
  total
}

fn char_to_priority(c:char) -> usize {
  if c.is_lowercase() {
    c as usize - 97 + 1 
  } else {
    c as usize - 65 + 26 + 1
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(157, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(70, part2(&get_input()));
  }
}