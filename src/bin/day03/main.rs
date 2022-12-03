/**/

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> i32
{
	input
		.lines()
		.map(|line| priority_of_shared_item(line))
		.sum()
}

fn priority_of_shared_item(line: &str) -> i32
{
	let n = line.len() / 2;
	let a = item_set_bitmask(&line[0..n]);
	let b = item_set_bitmask(&line[n..]);
	let shared = a & b;
	let offset = shared.trailing_zeros();
	let priority = 1 + offset as i32;
	priority
}

fn item_set_bitmask(compartment: &str) -> u64
{
	compartment.bytes().fold(0u64, |mut bitmask, item| {
		let offset = match item
		{
			b'a'..=b'z' => item - b'a',
			b'A'..=b'Z' => 26 + item - b'A',
			_ => unreachable!(),
		};
		bitmask |= 1 << offset;
		bitmask
	})
}

fn two(input: &str) -> i32
{
	input
		.lines()
		.chunks(3)
		.into_iter()
		.map(|group| {
			group.fold(!0u64, |bitmask, line| bitmask & item_set_bitmask(line))
		})
		.map(|badge| 1 + badge.trailing_zeros() as i32)
		.sum()
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
		assert_eq!(one(PROVIDED), 157);
	}

	#[test]
	fn one_test()
	{
		assert_eq!(one(TEST), 136);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 70);
	}
}
