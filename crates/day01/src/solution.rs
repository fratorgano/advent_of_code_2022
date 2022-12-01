use std::collections::BinaryHeap;

pub fn part1(input:&[String]) -> usize {
  let mut max = 0;
  let mut current_total = 0;
  for l in input{
    if l.is_empty() {
      if current_total > max {
        max = current_total
      }
      current_total = 0;
    } else {
      current_total += l.parse::<usize>().unwrap()
    }
  }
  max
}

pub fn part2(input:&[String]) -> usize {
  let mut heap = BinaryHeap::new();
  let mut current_total = 0;
  for l in input{
    if l.is_empty() {
      heap.push(current_total);
      current_total = 0;
    } else {
      current_total += l.parse::<usize>().unwrap()
    }
  }
  let mut sum = 0;
  for _ in 0..3 {
    let values = heap.pop().unwrap();
    sum += values;
  }
  sum
}
/* 
// Tried implementing another idea I had in mind but it's basically as fast as the previous
pub fn part2_2(input:&[String]) -> usize {
  let mut maxes = [0;4];
  let mut current_total = 0;
  for l in input{
    if l.is_empty() {
      maxes[0] = current_total;
      maxes.sort();
      current_total = 0;
    } else {
      current_total += l.parse::<usize>().unwrap()
    }
  }
  maxes[1] + maxes[2] + maxes[3]
}
*/

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000
    ".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(24000, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(45000, part2(&get_input()));
  }
}