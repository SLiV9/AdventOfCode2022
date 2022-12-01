/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> i32
{
	let mut max: i32 = 0;
	let mut current: i32 = 0;
	for line in input.lines()
	{
		if line.is_empty()
		{
			if current > max
			{
				max = current;
				current = 0;
			}
		}
		else
		{
			let calories: i32 = line.parse().unwrap();
			current += calories;
		}
	}
	max
}

fn two(_input: &str) -> i32
{
	0
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");

	#[test]
	fn one_test()
	{
		assert_eq!(one(PROVIDED), 24000);
	}
}
