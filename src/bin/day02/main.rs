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
#[repr(u8)]
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
#[repr(u8)]
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

#[derive(
	parse_display::Display, parse_display::FromStr, Debug, Clone, Copy,
)]
#[repr(u8)]
enum Outcome
{
	#[display("X")]
	Loss = 0,
	#[display("Y")]
	Draw = 1,
	#[display("Z")]
	Win = 2,
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

fn two(input: &str) -> i32
{
	input.lines().map(|line| calculate_alternative(line)).sum()
}

#[derive(parse_display::Display, parse_display::FromStr)]
#[display("{opponent} {outcome}")]
struct AlternativeRound
{
	opponent: Opponent,
	outcome: Outcome,
}

impl AlternativeRound
{
	fn response(&self) -> Response
	{
		match (3 + self.opponent as u8 + self.outcome as u8 - 1) % 3
		{
			0 => Response::Rock,
			1 => Response::Paper,
			_ => Response::Scissors,
		}
	}
}

fn calculate_alternative(line: &str) -> i32
{
	let round: AlternativeRound = line.parse().unwrap();
	round.outcome.value() + round.response().value()
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

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 12);
	}

	#[test]
	fn two_test()
	{
		assert_eq!(two(TEST), 18 + 9 + 3 + 6 + 9);
	}
}
