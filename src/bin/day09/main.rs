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
	let mut simulation: Simulation<2> = Simulation::new();
	for instruction in instructions
	{
		simulation.follow(&instruction);
	}
	simulation.positions_visited_by_tail.len()
}

fn two(input: &str) -> usize
{
	let instructions = input.lines().map(|x| x.parse().unwrap());
	let mut simulation: Simulation<10> = Simulation::new();
	for instruction in instructions
	{
		simulation.follow(&instruction);
	}
	simulation.positions_visited_by_tail.len()
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[display("{direction} {amount}")]
struct Instruction
{
	direction: Direction,
	amount: usize,
}

#[derive(
	Debug, Clone, Copy, parse_display::Display, parse_display::FromStr,
)]
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

impl Position
{
	fn step(&mut self, direction: Direction)
	{
		match direction
		{
			Direction::Up => self.y -= 1,
			Direction::Down => self.y += 1,
			Direction::Left => self.x -= 1,
			Direction::Right => self.x += 1,
		}
	}

	fn stay_attached_to(&mut self, head: Position)
	{
		let dx = head.x - self.x;
		let dy = head.y - self.y;
		let (dx, dy) = simulate_slack(dx, dy);
		self.x += dx;
		self.y += dy;
	}
}

struct Simulation<const N: usize>
{
	knots: [Position; N],
	positions_visited_by_tail: HashSet<Position>,
}

impl<const N: usize> Simulation<N>
{
	fn new() -> Simulation<N>
	{
		Simulation {
			knots: [Position::default(); N],
			positions_visited_by_tail: HashSet::from([Position::default()]),
		}
	}

	fn follow(&mut self, instruction: &Instruction)
	{
		for _t in 0..instruction.amount
		{
			self.knots[0].step(instruction.direction);
			for i in 1..N
			{
				self.knots[i].stay_attached_to(self.knots[i - 1]);
			}
			self.positions_visited_by_tail.insert(self.knots[N - 1]);
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
	const LARGER: &str = include_str!("larger.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 13);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 1);
	}

	#[test]
	fn two_larger()
	{
		assert_eq!(two(LARGER), 36);
	}
}
