/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

pub fn one(input: &str) -> i32
{
	input
		.lines()
		.map(|x| x.parse().unwrap())
		.fold(0, |acc: i32, x: i32| acc + x)
}

pub fn two(_input: &str) -> i32
{
	0
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const TEST: &str = include_str!("test.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one("+1\n-2\n+3\n+1"), 3);
	}

	#[test]
	fn one_test()
	{
		assert_eq!(one(TEST), 4);
	}
}
