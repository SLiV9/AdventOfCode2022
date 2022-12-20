/**/

use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> i32
{
	let numbers = input.lines().map(|line| line.parse().unwrap());
	let mut data: Vec<i32> = numbers.collect();
	let n = data.len();
	// Force uniqueness while keeping equality modulo n.
	let min_value = data.iter().min();
	let max_value = data.iter().max();
	dbg!(min_value);
	dbg!(max_value);
	let k = i32::try_from(10000 * n).unwrap();
	dbg!(k);
	let mut set: HashSet<i32> = HashSet::new();
	for value in data.iter_mut()
	{
		while set.insert(*value) == false
		{
			if *value > 0
			{
				*value += k;
			}
			else if *value < 0
			{
				*value -= k;
			}
			else
			{
				panic!("0 must be unique");
			}
		}
	}
	let commands = data.clone().into_iter();
	// Perform the mixing.
	for value in commands
	{
		let i = data.iter().position(|v| *v == value).unwrap();
		if value > 0
		{
			let shift: usize = usize::try_from(value).unwrap() % n;
			if i + shift < n
			{
				let from = i;
				let to = i + shift;
				data[from..=to].rotate_left(1);
			}
			else
			{
				let from = (i + shift + 1) % n;
				let to = i;
				data[from..=to].rotate_right(1);
			}
		}
		else if value < 0
		{
			let shift: usize = usize::try_from(-value).unwrap() % n;
			if i >= shift
			{
				let from = i - shift;
				let to = i;
				data[from..=to].rotate_right(1);
			}
			else
			{
				let from = i;
				let to = n + i - shift - 1;
				assert!(to < n);
				data[from..=to].rotate_left(1);
			}
		}
	}
	// Get the answer.
	let start = data.iter().position(|v| *v == 0).unwrap();
	let x = data[(start + 1000) % n] % k;
	let y = data[(start + 2000) % n] % k;
	let z = data[(start + 3000) % n] % k;
	x + y + z
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
	const DUPLICATE: &str = include_str!("duplicate.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 3);
	}

	#[test]
	fn one_duplicate()
	{
		// 1, 2, -3, 0, 3, -3, -2
		// 1, 2, -3, -3, 0, 3, -2
		assert_eq!(one(DUPLICATE), -4);
	}
}
