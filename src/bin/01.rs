advent_of_code::solution!(1);
use debug_print::debug_println;

pub fn part_one(input: &str) -> Option<u64> {
  let (directions, distances): (Vec<char>, Vec<i32>) = input
    .lines()
    .filter_map(|line| {
      let mut chars = line.chars();
      let first = chars.next()?;
      let remaining_str: String = chars.collect();
      let num = remaining_str.parse::<i32>().ok()?;
      Some((first, num))
    })
    .unzip();

  let mut rotation: i32 = 50;
  let mut count: u64 = 0;
  for (i, &distance) in distances.iter().enumerate() {
    debug_println!("Direction: {}, Distance: {}", directions[i], distance);
    rotation = match directions[i] {
      'L' => {
        if rotation < distance {
          ((rotation - distance) % 100 + 100) % 100
        } else {
          rotation - distance
        }
      }
      'R' => {
        if rotation + distance >= 100 {
          (rotation + distance) % 100
        } else {
          rotation + distance
        }
      }
      _ => panic!("Invalid direction"),
    };
    count = if rotation == 0 { count + 1 } else { count };
    debug_println!("Current Rotation: {}", rotation);
    if (rotation < 0) || (rotation > 99) {
      panic!("Rotation out of bounds: {}", rotation);
    }
  }
  Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
  let (directions, distances): (Vec<char>, Vec<i32>) = input
    .lines()
    .filter_map(|line| {
      let mut chars = line.chars();
      let first = chars.next()?;
      let remaining_str: String = chars.collect();
      let num = remaining_str.parse::<i32>().ok()?;
      Some((first, num))
    })
    .unzip();

  let mut rotation: i32 = 50;
  let mut count: u64 = 0;

  for (i, &distance) in distances.iter().enumerate() {
    debug_println!("Direction: {}, Distance: {}", directions[i], distance);
    let d = match directions[i] {
      'L' => -1,
      'R' => 1,
      _ => panic!("Invalid direction"),
    };
    for j in 0..distance {
      rotation = (rotation + d + 100) % 100;
      if rotation == 0 {
        count += 1;
      }
    }
  }

  Some(count)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(3));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(6));
  }
}
