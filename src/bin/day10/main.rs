/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> i32
{
	let mut total_signal_strength = 0;
	let mut register = 1;
	let mut cycle = 1;
	let mut next_important_cycle = 20;
	for instruction in input.lines().map(|line| line.parse().unwrap())
	{
		let (new_register, new_cycle) = match instruction
		{
			Instruction::Noop => (register, cycle + 1),
			Instruction::Addx { value } => (register + value, cycle + 2),
		};
		if new_cycle > next_important_cycle
		{
			total_signal_strength += next_important_cycle * register;
			next_important_cycle += 40;
		}
		register = new_register;
		cycle = new_cycle;
	}
	total_signal_strength
}

fn two(_input: &str) -> i32
{
	0
}

#[derive(
	Debug, Clone, Copy, parse_display::Display, parse_display::FromStr,
)]
enum Instruction
{
	#[display("noop")]
	Noop,
	#[display("addx {value}")]
	Addx
	{
		value: i32
	},
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 13140);
	}
}
