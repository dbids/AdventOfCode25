advent_of_code::solution!(2);
use debug_print::debug_println;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
  let re = Regex::new(r"(?<first>[0-9]+)-(?<last>[0-9]+)").unwrap();
  let mut sum: u64 = 0;

  // Iterate over input, one line at a time
  input.split(',').for_each(|line| {
    let Some(caps) = re.captures(line) else {
      panic!("No match for line: {}", line);
    };
    debug_println!("first {}, last {}", &caps["first"], &caps["last"]);

    // Iterate over the range between first and last
    let first = caps["first"].parse::<u64>().unwrap();
    let last = caps["last"].parse::<u64>().unwrap();
    for id in first..=last {
      // Convert id to string
      let id_str = id.to_string();

      // Check if string has an even number of chars
      if id_str.len() % 2 != 0 {
        continue;
      }

      // Split string in half and compare
      let (first_half, second_half) = id_str.split_at(id_str.len() / 2);
      if first_half == second_half {
        debug_println!("Found matching id: {}", id);
        sum += id;
      }
    }
  });

  return Some(sum);
}

pub fn part_two(input: &str) -> Option<u64> {
  let re = Regex::new(r"(?<first>[0-9]+)-(?<last>[0-9]+)").unwrap();
  let mut sum: u64 = 0;

  // Iterate over input, one line at a time
  input.split(',').for_each(|line| {
    let Some(caps) = re.captures(line) else {
      panic!("No match for line: {}", line);
    };
    debug_println!("first {}, last {}", &caps["first"], &caps["last"]);

    // Iterate over the range between first and last
    let first = caps["first"].parse::<u64>().unwrap();
    let last = caps["last"].parse::<u64>().unwrap();
    for id in first..=last {
      // Convert id to string
      let id_str = id.to_string();

      let mut match_found = false;

      // Loop over n, which represents the number of equal parts to split the string into
      for n in 1..=id_str.len() {
        // Remove if not divisible or match has already been found
        if (id_str.len() % n != 0) || (match_found == true) {
          continue;
        }

        // Split string into n equal parts
        let mut split_id_str: Vec<&str> = Vec::new();
        let split_size = id_str.len() / n;
        for m in 0..n {
          split_id_str.push(&id_str[split_size * m..split_size * (m + 1)]);
        }

        // Compare all the parts for equality
        debug_println!("Checking id {} with parts {:?}", id, split_id_str);
        if split_id_str.windows(2).all(|w| w[0] == w[1]) && (split_id_str.len() > 1) {
          match_found = true;
          break;
        }
      }

      // Sum if found
      if match_found {
        debug_println!("Found matching id: {}", id);
        sum += id;
      }
    }
  });

  return Some(sum);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(1227775554));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(4174379265));
  }
}
