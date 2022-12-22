/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

const INSTRUCTION_REGEX: &str = "(?P<steps>[0-9]+)|(?P<letter>[LR])";

fn one(input: &str) -> i32
{
	let (grid_input, instruction_input) = input.split_once("\n\n").unwrap();
	let instruction_regex = regex::Regex::new(INSTRUCTION_REGEX).unwrap();
	let puzzle = Puzzle::parse(grid_input);
	let mut finger = Finger::default();
	finger.find_start(&puzzle);
	for captured in instruction_regex.captures_iter(instruction_input)
	{
		match captured.name("letter").map(|x| x.as_str())
		{
			None =>
			{
				let steps_str = captured.name("steps").unwrap().as_str();
				let steps: u32 = steps_str.parse().unwrap();
				finger.walk(&puzzle, steps);
			}
			Some("L") =>
			{
				finger.facing = finger.facing.turned_counterclockwise();
			}
			Some("R") =>
			{
				finger.facing = finger.facing.turned_clockwise();
			}
			Some(_) => unreachable!(),
		}
	}
	finger.password()
}

fn two(_input: &str) -> i32
{
	0
}

#[derive(Debug, Clone, Copy)]
struct Finger
{
	row: usize,
	col: usize,
	facing: Facing,
}

impl Default for Finger
{
	fn default() -> Finger
	{
		Finger {
			row: 1,
			col: 1,
			facing: Facing::Right,
		}
	}
}

impl Finger
{
	fn find_start(&mut self, puzzle: &Puzzle)
	{
		while puzzle.grid[self.row][self.col] == 0
		{
			self.col += 1;
		}
	}

	fn walk(&mut self, puzzle: &Puzzle, steps: u32)
	{
		match self.facing
		{
			Facing::Right =>
			{
				for _step in 0..steps
				{
					if puzzle.grid[self.row][self.col + 1] == 0
					{
						while puzzle.grid[self.row][self.col - 1] != 0
						{
							self.col -= 1;
						}
					}
					else if puzzle.grid[self.row][self.col + 1] == b'#'
					{
						return;
					}
					else
					{
						self.col += 1;
					}
				}
			}
			Facing::Down =>
			{
				for _step in 0..steps
				{
					if puzzle.grid[self.row + 1][self.col] == 0
					{
						while puzzle.grid[self.row - 1][self.col] != 0
						{
							self.row -= 1;
						}
					}
					else if puzzle.grid[self.row + 1][self.col] == b'#'
					{
						return;
					}
					else
					{
						self.row += 1;
					}
				}
			}
			Facing::Left =>
			{
				for _step in 0..steps
				{
					if puzzle.grid[self.row][self.col - 1] == 0
					{
						while puzzle.grid[self.row][self.col + 1] != 0
						{
							self.col += 1;
						}
					}
					else if puzzle.grid[self.row][self.col - 1] == b'#'
					{
						return;
					}
					else
					{
						self.col -= 1;
					}
				}
			}
			Facing::Up =>
			{
				for _step in 0..steps
				{
					if puzzle.grid[self.row - 1][self.col] == 0
					{
						while puzzle.grid[self.row + 1][self.col] != 0
						{
							self.row += 1;
						}
					}
					else if puzzle.grid[self.row - 1][self.col] == b'#'
					{
						return;
					}
					else
					{
						self.row -= 1;
					}
				}
			}
		}
	}

	fn password(&self) -> i32
	{
		let r = self.row as i32;
		let c = self.col as i32;
		1000 * r + 4 * c + self.facing.password()
	}
}

#[derive(Debug, Clone, Copy)]
enum Facing
{
	Right,
	Down,
	Left,
	Up,
}

impl Facing
{
	fn turned_counterclockwise(self) -> Self
	{
		match self
		{
			Facing::Right => Facing::Up,
			Facing::Down => Facing::Right,
			Facing::Left => Facing::Down,
			Facing::Up => Facing::Left,
		}
	}

	fn turned_clockwise(self) -> Self
	{
		match self
		{
			Facing::Right => Facing::Down,
			Facing::Down => Facing::Left,
			Facing::Left => Facing::Up,
			Facing::Up => Facing::Right,
		}
	}

	fn password(self) -> i32
	{
		match self
		{
			Facing::Right => 0,
			Facing::Down => 1,
			Facing::Left => 2,
			Facing::Up => 3,
		}
	}
}

// The puzzle grid is surrounded by 0 bytes which indicate the void.
// This also means we do not have to do any bounds checking.
const GRID_WIDTH: usize = 202;
const GRID_HEIGHT: usize = 202;

#[derive(Debug)]
struct Puzzle
{
	grid: [[u8; GRID_WIDTH]; GRID_HEIGHT],
}

impl Puzzle
{
	fn parse(input: &str) -> Puzzle
	{
		let mut grid = [[0; GRID_WIDTH]; GRID_HEIGHT];
		for (i, line) in input.lines().enumerate()
		{
			let row = 1 + i;
			assert!(row + 1 < GRID_HEIGHT);
			assert!(line.len() + 2 < GRID_WIDTH);
			for (j, x) in line.bytes().enumerate()
			{
				let col = 1 + j;
				match x
				{
					b' ' => (),
					b'.' | b'#' => grid[row][col] = x,
					_ => unreachable!(),
				}
			}
		}
		Puzzle { grid }
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
		assert_eq!(one(PROVIDED), 6032);
	}
}
