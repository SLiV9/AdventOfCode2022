/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> i32
{
	input.lines().map(|line| calculate_score(line)).sum()
}

#[derive(parse_display::Display, parse_display::FromStr)]
#[display("{opponent} {response}")]
struct Round
{
	opponent: Opponent,
	response: Response,
}

impl Round
{
	fn outcome(&self) -> Outcome
	{
		match (3 + self.response as u8 - self.opponent as u8) % 3
		{
			0 => Outcome::Draw,
			1 => Outcome::Win,
			_ => Outcome::Loss,
		}
	}
}

#[derive(
	Debug, Clone, Copy, parse_display::Display, parse_display::FromStr,
)]
enum Opponent
{
	#[display("A")]
	Rock = 0,
	#[display("B")]
	Paper = 1,
	#[display("C")]
	Scissors = 2,
}

#[derive(
	Debug, Clone, Copy, parse_display::Display, parse_display::FromStr,
)]
enum Response
{
	#[display("X")]
	Rock = 0,
	#[display("Y")]
	Paper = 1,
	#[display("Z")]
	Scissors = 2,
}

impl Response
{
	fn value(self) -> i32
	{
		match self
		{
			Response::Rock => 1,
			Response::Paper => 2,
			Response::Scissors => 3,
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum Outcome
{
	Loss,
	Draw,
	Win,
}

impl Outcome
{
	fn value(self) -> i32
	{
		match self
		{
			Outcome::Loss => 0,
			Outcome::Draw => 3,
			Outcome::Win => 6,
		}
	}
}

fn calculate_score(line: &str) -> i32
{
	let round: Round = line.parse().unwrap();
	round.outcome().value() + round.response.value()
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
	const TEST: &str = include_str!("test.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 15);
	}

	#[test]
	fn one_test()
	{
		assert_eq!(one(TEST), 18 + 9 + 3 + 6 + 9);
	}
}
