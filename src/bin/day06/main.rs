/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> usize
{
	find_consecutive_unique(input, 4)
}

fn find_consecutive_unique(input: &str, num: usize) -> usize
{
	let (i, _window) = input
		.as_bytes()
		.windows(num)
		.enumerate()
		.find(|(_i, window)| all_unique(window))
		.unwrap();
	num + i
}

fn all_unique(code: &[u8]) -> bool
{
	let mut bitmask = 0u32;
	for letter in code
	{
		let offset = letter - b'a';
		let bit = 1 << offset;
		if bitmask & bit != 0
		{
			return false;
		}
		bitmask = bitmask | bit;
	}
	true
}

fn two(input: &str) -> usize
{
	find_consecutive_unique(input, 14)
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	#[test]
	fn one_provided()
	{
		assert_eq!(one("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
		assert_eq!(one("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
		assert_eq!(one("nppdvjthqldpwncqszvftbrmjlhg"), 6);
		assert_eq!(one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
		assert_eq!(one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
		assert_eq!(two("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
		assert_eq!(two("nppdvjthqldpwncqszvftbrmjlhg"), 23);
		assert_eq!(two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
		assert_eq!(two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
	}
}
