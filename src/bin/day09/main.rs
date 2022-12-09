/**/

use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> usize
{
	let instructions = input.lines().map(|x| x.parse().unwrap());
	let mut simulation = Simulation::new();
	for instruction in instructions
	{
		simulation.follow(&instruction);
	}
	simulation.positions_visited_by_tail.len()
}

fn two(_input: &str) -> usize
{
	0
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[display("{direction} {amount}")]
struct Instruction
{
	direction: Direction,
	amount: usize,
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
enum Direction
{
	#[display("U")]
	Up,
	#[display("D")]
	Down,
	#[display("L")]
	Left,
	#[display("R")]
	Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Position
{
	x: i32,
	y: i32,
}

struct Simulation
{
	head: Position,
	tail: Position,
	positions_visited_by_tail: HashSet<Position>,
}

impl Simulation
{
	fn new() -> Simulation
	{
		Simulation {
			head: Position::default(),
			tail: Position::default(),
			positions_visited_by_tail: HashSet::from([Position::default()]),
		}
	}

	fn follow(&mut self, instruction: &Instruction)
	{
		for _i in 0..instruction.amount
		{
			match instruction.direction
			{
				Direction::Up => self.head.y -= 1,
				Direction::Down => self.head.y += 1,
				Direction::Left => self.head.x -= 1,
				Direction::Right => self.head.x += 1,
			}
			let dx = self.head.x - self.tail.x;
			let dy = self.head.y - self.tail.y;
			let (dx, dy) = simulate_slack(dx, dy);
			self.tail.x += dx;
			self.tail.y += dy;
			self.positions_visited_by_tail.insert(self.tail);
		}
	}
}

fn simulate_slack(dx: i32, dy: i32) -> (i32, i32)
{
	match (dx, dy)
	{
		(-2, -2) => (-1, -1),
		(-2, -1) => (-1, -1),
		(-2, 0) => (-1, 0),
		(-2, 1) => (-1, 1),
		(-2, 2) => (-1, 1),
		(-1, -2) => (-1, -1),
		(-1, -1) => (0, 0),
		(-1, 0) => (0, 0),
		(-1, 1) => (0, 0),
		(-1, 2) => (-1, 1),
		(0, -2) => (0, -1),
		(0, -1) => (0, 0),
		(0, 0) => (0, 0),
		(0, 1) => (0, 0),
		(0, 2) => (0, 1),
		(1, -2) => (1, -1),
		(1, -1) => (0, 0),
		(1, 0) => (0, 0),
		(1, 1) => (0, 0),
		(1, 2) => (1, 1),
		(2, -2) => (1, -1),
		(2, -1) => (1, -1),
		(2, 0) => (1, 0),
		(2, 1) => (1, 1),
		(2, 2) => (1, 1),
		(dx, dy) =>
		{
			eprintln!("Snap!");
			(dx, dy)
		}
	}
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
		assert_eq!(one(PROVIDED), 13);
	}
}
