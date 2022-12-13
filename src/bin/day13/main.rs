/**/

use std::cmp::Ordering;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> usize
{
	input
		.split("\n\n")
		.enumerate()
		.map(|(i, input)| (i, separate_package_inputs(input)))
		.filter(|(_i, (a, b))| are_in_right_order(a, b))
		.map(|(i, _)| 1 + i)
		.sum()
}

fn two(input: &str) -> usize
{
	// We can cheat a litte: we only need to know the indices of the dividers,
	// so we can instead just calculate the number of lines that are earlier.
	let mut num_less_than_2 = 0;
	let mut num_between = 0;
	for line in input.lines().filter(|x| !x.is_empty())
	{
		match compare_to_range(line, 2, 6)
		{
			Ordering::Less => num_less_than_2 += 1,
			Ordering::Equal => num_between += 1,
			Ordering::Greater => (),
		}
	}
	let offset_of_2 = 1 + num_less_than_2;
	let offset_of_6 = offset_of_2 + num_between + 1;
	offset_of_2 * offset_of_6
}

fn separate_package_inputs(input: &str) -> (&str, &str)
{
	let mut lines = input.lines();
	let a = lines.next().unwrap();
	let b = lines.next().unwrap();
	assert_eq!(lines.next(), None);
	(a, b)
}

fn are_in_right_order(left: &str, right: &str) -> bool
{
	match compare(left, right)
	{
		Ordering::Less => true,
		Ordering::Greater => false,
		Ordering::Equal => unreachable!(),
	}
}

fn compare_to_range(line: &str, lower: i32, upper: i32) -> Ordering
{
	match compare_1(lower, &mut parse_tokens(line).skip(1)).reverse()
	{
		Ordering::Less => Ordering::Less,
		Ordering::Greater =>
		{
			match compare_1(upper, &mut parse_tokens(line).skip(1)).reverse()
			{
				Ordering::Less => Ordering::Equal,
				Ordering::Greater => Ordering::Greater,
				Ordering::Equal => Ordering::Equal,
			}
		}
		Ordering::Equal => Ordering::Equal,
	}
}

fn compare(left: &str, right: &str) -> Ordering
{
	let upper_bound = left.len() + right.len();
	let mut left = parse_tokens(left);
	let mut right = parse_tokens(right);
	for _ in 0..upper_bound
	{
		let order = match (left.next().unwrap(), right.next().unwrap())
		{
			(Token::Num(a), Token::Num(b)) => a.cmp(&b),
			(Token::Num(a), Token::Open) => compare_1(a, &mut right),
			(Token::Open, Token::Num(b)) => compare_1(b, &mut left).reverse(),
			(Token::Open, Token::Open) => Ordering::Equal,
			(Token::Close, Token::Close) => Ordering::Equal,
			(Token::Close, _) => Ordering::Less,
			(_, Token::Close) => Ordering::Greater,
		};
		match order
		{
			Ordering::Equal => (),
			Ordering::Less => return Ordering::Less,
			Ordering::Greater => return Ordering::Greater,
		}
	}
	Ordering::Equal
}

fn compare_1(a: i32, right: &mut impl Iterator<Item = Token>) -> Ordering
{
	let mut inner_depth = 1;
	while inner_depth > 0
	{
		match right.next().unwrap()
		{
			Token::Open => inner_depth += 1,
			Token::Close => return Ordering::Greater,
			Token::Num(b) if a < b => return Ordering::Less,
			Token::Num(b) if a > b => return Ordering::Greater,
			Token::Num(_) => break,
		}
	}
	// We have found a == b within some number of nested brackets.
	// We need to return to the surface, unless one of the inner
	// lists contains a second number, in which case a is first.
	while inner_depth > 0
	{
		match right.next().unwrap()
		{
			Token::Open => inner_depth += 1,
			Token::Close => inner_depth -= 1,
			Token::Num(_) => return Ordering::Less,
		}
	}
	Ordering::Equal
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token
{
	Open,
	Close,
	Num(i32),
}

fn parse_tokens(input: &str) -> impl Iterator<Item = Token> + '_
{
	// The only multi-byte token is Num, so we can parse with a running value.
	// The last byte (']') is discarded, so we append it at the end.
	let mut state = None;
	input
		.as_bytes()
		.windows(2)
		.map(move |window| scan_token(&mut state, window[0], window[1]))
		.filter_map(|x| x)
		.chain(std::iter::once(Token::Close))
}

fn scan_token(
	accumulator: &mut Option<i32>,
	current_symbol: u8,
	next_symbol: u8,
) -> Option<Token>
{
	let acc = accumulator.take();
	match current_symbol
	{
		b'[' => Some(Token::Open),
		b']' => Some(Token::Close),
		b',' => None,
		b' ' => None,
		x if x.is_ascii_digit() =>
		{
			let digit_value: i32 = (x - b'0').into();
			let value: i32 = acc.unwrap_or(0) * 10 + digit_value;
			if next_symbol.is_ascii_digit()
			{
				*accumulator = Some(value);
				None
			}
			else
			{
				Some(Token::Num(value))
			}
		}
		_ => unreachable!(),
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");

	#[test]
	fn scan_sample()
	{
		assert_eq!(
			parse_tokens("[[1],[2,3,4]]").collect::<Vec<Token>>(),
			[
				Token::Open,
				Token::Open,
				Token::Num(1),
				Token::Close,
				Token::Open,
				Token::Num(2),
				Token::Num(3),
				Token::Num(4),
				Token::Close,
				Token::Close
			]
		);
	}

	#[test]
	fn right_order()
	{
		assert!(are_in_right_order("[1]", "[[[[[2]]]]]"));
	}

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 13);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 140);
	}
}
