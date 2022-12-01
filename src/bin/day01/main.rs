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
			}
			current = 0;
		}
		else
		{
			let calories: i32 = line.parse().unwrap();
			current += calories;
		}
	}
	if current > max
	{
		max = current;
	}
	max
}

fn two(input: &str) -> i32
{
	let mut top_four: [i32; 4] = [0; 4];
	let mut current: i32 = 0;
	for line in input.lines()
	{
		if line.is_empty()
		{
			top_four[0] = current;
			top_four.sort();
			current = 0;
		}
		else
		{
			let calories: i32 = line.parse().unwrap();
			current += calories;
		}
	}
	top_four[0] = current;
	top_four.sort();
	top_four[1] + top_four[2] + top_four[3]
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");
	const TEST: &str = include_str!("test.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 24000);
	}

	#[test]
	fn one_test()
	{
		assert_eq!(one(TEST), 24001);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 45000);
	}

	#[test]
	fn two_test()
	{
		assert_eq!(two(TEST), 59001);
	}
}
