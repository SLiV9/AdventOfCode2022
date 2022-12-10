/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: \n{}", two(INPUT));
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

const SCREEN_HEIGHT: usize = 6;
const SCREEN_WIDTH: usize = 40;
const SCREEN_STRIDE: usize = SCREEN_WIDTH + 1;

fn two(input: &str) -> String
{
	let mut screen = [b'.'; SCREEN_HEIGHT * SCREEN_STRIDE];
	for r in 0..SCREEN_HEIGHT
	{
		screen[r * SCREEN_STRIDE + SCREEN_WIDTH] = b'\n';
	}
	let mut register = 1;
	let mut cycle = 1;
	for instruction in input.lines().map(|line| line.parse().unwrap())
	{
		let (new_register, new_cycle) = match instruction
		{
			Instruction::Noop => (register, cycle + 1),
			Instruction::Addx { value } => (register + value, cycle + 2),
		};
		for t in cycle..new_cycle
		{
			let r = (t as usize - 1) / SCREEN_WIDTH;
			let c = (t as usize - 1) % SCREEN_WIDTH;
			let draw_col = c as i32;
			if draw_col >= register - 1 && draw_col <= register + 1
			{
				screen[r * SCREEN_STRIDE + c] = b'#';
			}
		}
		register = new_register;
		cycle = new_cycle;
	}
	std::str::from_utf8(&screen).unwrap().to_string()
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
	const PROVIDED_OUTPUT: &str = include_str!("provided_output.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 13140);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), PROVIDED_OUTPUT);
	}
}
