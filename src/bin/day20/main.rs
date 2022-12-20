/**/

use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> i64
{
	let numbers = input.lines().map(|line| line.parse().unwrap());
	let mut data: Vec<i64> = numbers.collect();
	let n = data.len() - 1;
	// Force uniqueness while keeping equality modulo n.
	let min_value = data.iter().min().unwrap();
	let max_value = data.iter().max().unwrap();
	dbg!(min_value);
	dbg!(max_value);
	let largest_value = std::cmp::max(min_value.abs(), max_value.abs());
	let k = largest_value * i64::try_from(n).unwrap();
	dbg!(k);
	make_unique(&mut data, k);
	let commands = data.clone().into_iter();
	// Perform the mixing.
	for value in commands
	{
		mix_1(value, &mut data);
	}
	// Get the answer.
	let start = data.iter().position(|v| *v == 0).unwrap();
	let x = data[(start + 1000) % data.len()] % k;
	let y = data[(start + 2000) % data.len()] % k;
	let z = data[(start + 3000) % data.len()] % k;
	dbg!(x) + dbg!(y) + dbg!(z)
}

const DECRYPTION_KEY: i64 = 811589153;

fn two(input: &str) -> i64
{
	let numbers = input.lines().map(|line| line.parse().unwrap());
	let mut data: Vec<i64> = numbers.collect();
	let n = data.len() - 1;
	// Multiply by decryption key.
	for value in data.iter_mut()
	{
		*value *= DECRYPTION_KEY;
	}
	// Force uniqueness while keeping equality modulo n.
	let min_value = data.iter().min().unwrap();
	let max_value = data.iter().max().unwrap();
	dbg!(min_value);
	dbg!(max_value);
	let largest_value = std::cmp::max(min_value.abs(), max_value.abs());
	let k = largest_value * i64::try_from(n).unwrap();
	dbg!(k);
	make_unique(&mut data, k);
	let commands = data.clone();
	// Perform the mixing.
	for _ in 0..10
	{
		for value in &commands
		{
			mix_1(*value, &mut data);
		}
	}
	// Get the answer.
	let start = data.iter().position(|v| *v == 0).unwrap();
	let x = data[(start + 1000) % data.len()];
	let y = data[(start + 2000) % data.len()];
	let z = data[(start + 3000) % data.len()];
	dbg!(x) + dbg!(y) + dbg!(z)
}

fn mix_1(value: i64, data: &mut [i64])
{
	let n = data.len() - 1;
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
			let from = (i + shift) % n;
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
			let to = n + i - shift;
			assert!(to < n);
			data[from..=to].rotate_left(1);
		}
	}
}

fn make_unique(data: &mut [i64], k: i64)
{
	let mut set: HashSet<i64> = HashSet::new();
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
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");
	const DUPLICATE: &str = include_str!("duplicate.txt");
	const NINE: &str = include_str!("nine.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 3);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 1623178306);
	}

	#[test]
	fn one_duplicate()
	{
		// 1, 2, -3, 0, 3, -3, -2
		// 1, 2, -3, -3, 0, 3, -2
		assert_eq!(one(DUPLICATE), -4);
	}

	#[test]
	fn one_nine()
	{
		assert_eq!(one(NINE), 0);
	}

	#[test]
	fn various_shifts()
	{
		assert_eq!(shifted([1, 2, 3], 1), [2, 1, 3]);
		assert_eq!(shifted([1, 2, 3], 2), [1, 2, 3]);
		assert_eq!(shifted([1, 2, 3], 3), [1, 3, 2]);
		assert_eq!(shifted([1, 4, 0], 4), [1, 4, 0]);
		assert_eq!(shifted([1, 5, 0], 5), [5, 1, 0]);
		assert_eq!(shifted([1, 6, 0], 6), [1, 6, 0]);
		assert_eq!(shifted([1, 7, 0], 7), [7, 1, 0]);
		assert_eq!(shifted([-1, 2, 3], -1), [2, -1, 3]);
		assert_eq!(shifted([1, -2, 3], -2), [1, -2, 3]);
		assert_eq!(shifted([1, 2, -3], -3), [1, -3, 2]);
		assert_eq!(shifted([1, -4, 0], -4), [1, -4, 0]);
		assert_eq!(shifted([1, -5, 0], -5), [-5, 1, 0]);
		assert_eq!(shifted([1, -6, 0], -6), [1, -6, 0]);
		assert_eq!(shifted([1, -7, 0], -7), [-7, 1, 0]);
	}

	fn shifted<const N: usize>(mut data: [i64; N], value: i64) -> [i64; N]
	{
		mix_1(value, &mut data);
		data
	}
}
