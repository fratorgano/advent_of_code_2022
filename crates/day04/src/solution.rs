pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  parsed.iter().filter(|x| is_complete_overlap(x)).count()
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  parsed.iter().filter(|x| is_partial_overlap(x)).count()
}

#[derive(Debug)]
struct Pair {
  first: (usize,usize),
  second: (usize,usize)
}

fn is_complete_overlap(pair:&Pair) -> bool {
  (pair.first.0 <= pair.second.0 && pair.first.1 >= pair.second.1) || 
  (pair.second.0 <= pair.first.0 && pair.second.1 >= pair.first.1)
}

fn is_partial_overlap(pair:&Pair) -> bool {
  // 5-7,7-9
  pair.first.0.max(pair.second.0) <= pair.first.1.min(pair.second.1)
}

fn parse(strings:&[String]) -> Vec<Pair> {
  let mut pairs = vec![];
  for s in strings {
    let mut splitted = s.split(',');
    let pair1:Vec<usize> = splitted.next().unwrap().split('-').map(|x| x.parse().unwrap()).collect();
    let pair2:Vec<usize> = splitted.next().unwrap().split('-').map(|x| x.parse().unwrap()).collect();
    pairs.push(Pair {first:(pair1[0],pair1[1]),second:(pair2[0],pair2[1])});
  }
  return pairs;
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(2, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(4, part2(&get_input()));
  }
}