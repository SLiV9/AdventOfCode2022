/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> String
{
	let (diagram, rest_of_input) = input.split_once("\n\n").unwrap();
	let mut ship = Ship::new();
	ship.fill(diagram);
	let instructions = rest_of_input.lines().map(|line| line.parse().unwrap());
	for instruction in instructions
	{
		ship.follow(instruction);
	}
	ship.get_message()
}

const MAX_WIDTH: usize = 9;
const MAX_HEIGHT: usize = 100;

struct Ship
{
	crates: [[u8; MAX_HEIGHT]; MAX_WIDTH],
	heights: [usize; MAX_WIDTH],
	width: usize,
}

impl Ship
{
	fn new() -> Ship
	{
		Ship {
			crates: [[b' '; MAX_HEIGHT]; MAX_WIDTH],
			heights: [0; MAX_WIDTH],
			width: 0,
		}
	}

	fn fill(&mut self, input: &str)
	{
		let mut h = input.lines().count();
		assert!(h >= 2);
		h -= 1;
		let mut lines = input.lines();
		let first_line = lines.next().unwrap();
		assert_eq!(first_line.len() % 4, 3);
		assert_eq!(input.len(), (h + 1) * (first_line.len() + 1) - 1);
		self.width = (first_line.len() + 1) / 4;
		for i in 0..self.width
		{
			self.crates[i][h] = first_line.as_bytes()[4 * i + 1];
		}
		for line in lines.take(h)
		{
			h -= 1;
			for i in 0..self.width
			{
				self.crates[i][h] = line.as_bytes()[4 * i + 1];
			}
		}
		for i in 0..self.width
		{
			self.heights[i] =
				self.crates[i].iter().position(|x| *x == b' ').unwrap();
		}
	}

	fn follow(&mut self, instruction: Instruction)
	{
		let Instruction {
			from_num,
			to_num,
			amount,
		} = instruction;
		assert!(from_num >= 1 && from_num as usize <= self.width);
		assert!(to_num >= 1 && to_num as usize <= self.width);
		assert_ne!(from_num, to_num);
		let from = from_num as usize - 1;
		let to = to_num as usize - 1;
		for _step in 0..amount
		{
			assert!(self.heights[from] > 0);
			self.crates[to][self.heights[to]] =
				self.crates[from][self.heights[from] - 1];
			self.heights[to] += 1;
			self.crates[from][self.heights[from] - 1] = b' ';
			self.heights[from] -= 1;
		}
	}

	fn get_message(&self) -> String
	{
		let top_crates: Vec<u8> = self.heights[0..self.width]
			.iter()
			.enumerate()
			.filter(|(_i, h)| **h > 0)
			.map(|(i, h)| self.crates[i][h - 1])
			.collect();
		String::from_utf8(top_crates).unwrap()
	}
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[display("move {amount} from {from_num} to {to_num}")]
struct Instruction
{
	from_num: i32,
	to_num: i32,
	amount: i32,
}

fn two(_input: &str) -> String
{
	String::new()
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");
	const AS_IS: &str = include_str!("as_is.txt");
	const ONCE: &str = include_str!("once.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(&one(PROVIDED), "CMZ");
	}

	#[test]
	fn one_as_is()
	{
		assert_eq!(&one(AS_IS), "NDP");
	}

	#[test]
	fn one_once()
	{
		assert_eq!(&one(ONCE), "DCP");
	}
}
