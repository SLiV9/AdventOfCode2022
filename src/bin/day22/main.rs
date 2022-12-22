/**/

const INPUT: &str = include_str!("input.txt");
const INPUT_RADIUS: usize = 50;

pub fn main()
{
	println!("Part One: {}", one(INPUT, INPUT_RADIUS));
	println!("Part Two: {}", two(INPUT, INPUT_RADIUS));
}

const INSTRUCTION_REGEX: &str = "(?P<steps>[0-9]+)|(?P<letter>[LR])";

fn one(input: &str, radius: usize) -> i32
{
	let (grid_input, instruction_input) = input.split_once("\n\n").unwrap();
	one_1(radius, grid_input, instruction_input)
}

fn one_1(radius: usize, grid_input: &str, instruction_input: &str) -> i32
{
	let puzzle = Puzzle::parse(grid_input, radius, false);
	solve(puzzle, instruction_input)
}

fn solve(puzzle: Puzzle, instruction_input: &str) -> i32
{
	let mut finger = Finger::default();
	let instruction_regex = regex::Regex::new(INSTRUCTION_REGEX).unwrap();
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
	finger.password(&puzzle)
}

fn two(input: &str, radius: usize) -> i32
{
	let (grid_input, instruction_input) = input.split_once("\n\n").unwrap();
	let puzzle = Puzzle::parse(grid_input, radius, true);
	solve(puzzle, instruction_input)
}

#[derive(Debug, Clone, Copy)]
struct Finger
{
	side: usize,
	row: usize,
	col: usize,
	facing: Facing,
}

impl Default for Finger
{
	fn default() -> Finger
	{
		Finger {
			side: 0,
			row: 1,
			col: 1,
			facing: Facing::Right,
		}
	}
}

impl Finger
{
	fn walk(&mut self, puzzle: &Puzzle, steps: u32)
	{
		for _step in 0..steps
		{
			self.step(puzzle)
		}
	}

	fn step(&mut self, puzzle: &Puzzle)
	{
		let (r, c) = match self.facing
		{
			Facing::Right => (self.row, self.col + 1),
			Facing::Down => (self.row + 1, self.col),
			Facing::Left => (self.row, self.col - 1),
			Facing::Up => (self.row - 1, self.col),
		};
		match puzzle.sides[self.side][r][c]
		{
			b'.' =>
			{
				self.row = r;
				self.col = c;
			}
			b'#' => (),
			code =>
			{
				let new_side = usize::from(((code & 0xF0) - 0xA0) >> 4);
				let new_facing = match code & 0x0F
				{
					0 => Facing::Right,
					1 => Facing::Down,
					2 => Facing::Left,
					3 => Facing::Up,
					_ => unreachable!(),
				};
				let max = puzzle.radius;
				let (new_row, new_col) = match (self.facing, new_facing)
				{
					(Facing::Right, Facing::Right) => (r, 1),
					(Facing::Down, Facing::Right) => (max + 1 - c, 1),
					(Facing::Left, Facing::Right) => (max + 1 - r, 1),
					(Facing::Up, Facing::Right) => (c, 1),
					(Facing::Right, Facing::Down) => (1, max + 1 - r),
					(Facing::Down, Facing::Down) => (1, c),
					(Facing::Left, Facing::Down) => (1, r),
					(Facing::Up, Facing::Down) => (1, max + 1 - c),
					(Facing::Right, Facing::Left) => (max + 1 - r, max),
					(Facing::Down, Facing::Left) => (c, max),
					(Facing::Left, Facing::Left) => (r, max),
					(Facing::Up, Facing::Left) => (max + 1 - c, max),
					(Facing::Right, Facing::Up) => (max, r),
					(Facing::Down, Facing::Up) => (max, max + 1 - c),
					(Facing::Left, Facing::Up) => (max, max + 1 - r),
					(Facing::Up, Facing::Up) => (max, c),
				};
				match puzzle.sides[new_side][new_row][new_col]
				{
					b'.' =>
					{
						self.side = new_side;
						self.facing = new_facing;
						self.row = new_row;
						self.col = new_col;
					}
					b'#' => (),
					_ => unreachable!(),
				}
				//dbg!(code);
				//dbg!(self.side);
				//dbg!(self.facing);
			}
		}
	}

	fn password(&self, puzzle: &Puzzle) -> i32
	{
		let (r, c) = puzzle.get_absolute_rc(self.side, self.row, self.col);
		dbg!((self.side, self.row, self.col, r, c, self.facing));
		1000 * (r as i32) + 4 * (c as i32) + i32::from(u8::from(self.facing))
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
}

impl From<Facing> for u8
{
	fn from(facing: Facing) -> u8
	{
		match facing
		{
			Facing::Right => 0,
			Facing::Down => 1,
			Facing::Left => 2,
			Facing::Up => 3,
		}
	}
}

// Each side grid is surrounded by bytes that indicate the wrap-around.
// This also means we do not have to do any bounds checking.
const GRID_SIZE: usize = 64;
const NUM_SIDES: usize = 6;

#[derive(Debug)]
struct Puzzle
{
	sides: [[[u8; GRID_SIZE]; GRID_SIZE]; NUM_SIDES],
	chunk_rc_of_side: [(usize, usize); NUM_SIDES],
	radius: usize,
}

impl Puzzle
{
	fn parse(input: &str, radius: usize, is_cube: bool) -> Puzzle
	{
		assert!(radius >= 2);
		assert!(radius + 2 <= GRID_SIZE);
		let mut sides = [[[0; GRID_SIZE]; GRID_SIZE]; NUM_SIDES];
		let mut chunk_rc_of_side = [(0, 0); NUM_SIDES];
		for (i, line) in input.lines().enumerate()
		{
			let chunk_r = i / radius;
			let row = 1 + i % radius;
			assert!(row + 1 < GRID_SIZE);
			for (chunk_c, chunk) in line.as_bytes().chunks(radius).enumerate()
			{
				if chunk.iter().all(|x| *x == b' ')
				{
					continue;
				}
				assert!(chunk.len() + 2 < GRID_SIZE);
				let side = chunk_rc_of_side
					.iter()
					.position(|&(r, c)| {
						(r, c) == (chunk_r, chunk_c) || (r, c) == (0, 0)
					})
					.unwrap();
				chunk_rc_of_side[side] = (chunk_r, chunk_c);
				for (j, x) in chunk.iter().enumerate()
				{
					let col = 1 + j;
					let glyph = *x;
					match glyph
					{
						b' ' => (),
						b'.' | b'#' => sides[side][row][col] = glyph,
						_ => unreachable!(),
					}
				}
			}
		}
		//dbg!(chunk_rc_of_side);
		if is_cube
		{
			// Part two: cube.
			for s in 0..NUM_SIDES
			{
				// Hardcoded for now:
				let conf = if radius == 50
				{
					&INPUT_CUBE_CONFIGURATION[s]
				}
				else
				{
					&PROVIDED_CUBE_CONFIGURATION[s]
				};
				// Get edges based on determined configuration.
				let left = conf.left.encode();
				let right = conf.right.encode();
				let up = conf.up.encode();
				let down = conf.down.encode();
				for r in 1..=radius
				{
					sides[s][r][0] = left;
					sides[s][r][radius + 1] = right;
				}
				for c in 1..=radius
				{
					sides[s][0][c] = up;
				}
				for c in 1..=radius
				{
					sides[s][radius + 1][c] = down;
				}
			}
		}
		else
		{
			// Part one: wrap-around.
			for s in 0..NUM_SIDES
			{
				let (chunk_r, chunk_c) = chunk_rc_of_side[s];
				let right = chunk_rc_of_side
					.iter()
					.enumerate()
					.fold(None, |best, (i, &(r, c))| {
						Some(i)
							.filter(|_i| (r, c) == (chunk_r, chunk_c + 1))
							.or(best)
							.or(Some(i).filter(|_i| r == chunk_r))
					})
					.map(|side| Edge {
						side,
						facing: Facing::Right,
					})
					.map(|edge| edge.encode())
					.unwrap();
				let down = chunk_rc_of_side
					.iter()
					.enumerate()
					.fold(None, |best, (i, &(r, c))| {
						Some(i)
							.filter(|_i| (r, c) == (chunk_r + 1, chunk_c))
							.or(best)
							.or(Some(i).filter(|_i| c == chunk_c))
					})
					.map(|side| Edge {
						side,
						facing: Facing::Down,
					})
					.map(|edge| edge.encode())
					.unwrap();
				let left = chunk_rc_of_side
					.iter()
					.enumerate()
					.rev()
					.fold(None, |best, (i, &(r, c))| {
						Some(i)
							.filter(|_i| (r, c + 1) == (chunk_r, chunk_c))
							.or(best)
							.or(Some(i).filter(|_i| r == chunk_r))
					})
					.map(|side| Edge {
						side,
						facing: Facing::Left,
					})
					.map(|edge| edge.encode())
					.unwrap();
				let up = chunk_rc_of_side
					.iter()
					.enumerate()
					.rev()
					.fold(None, |best, (i, &(r, c))| {
						Some(i)
							.filter(|_i| (r + 1, c) == (chunk_r, chunk_c))
							.or(best)
							.or(Some(i).filter(|_i| c == chunk_c))
					})
					.map(|side| Edge {
						side,
						facing: Facing::Up,
					})
					.map(|edge| edge.encode())
					.unwrap();
				for r in 1..=radius
				{
					sides[s][r][0] = left;
					sides[s][r][radius + 1] = right;
				}
				for c in 1..=radius
				{
					sides[s][0][c] = up;
				}
				for c in 1..=radius
				{
					sides[s][radius + 1][c] = down;
				}
			}
		}
		dbg_print_sides(&sides, radius);
		Puzzle {
			sides,
			chunk_rc_of_side,
			radius,
		}
	}

	fn get_absolute_rc(
		&self,
		side: usize,
		row: usize,
		col: usize,
	) -> (usize, usize)
	{
		let (rr, cc) = self.chunk_rc_of_side[side];
		let r = rr * self.radius + row;
		let c = cc * self.radius + col;
		(r, c)
	}
}

fn encode(side: usize, facing: Facing) -> u8
{
	0xA0 + (u8::try_from(side).unwrap() << 4) + u8::from(facing)
}

struct Configuration
{
	left: Edge,
	right: Edge,
	up: Edge,
	down: Edge,
}

#[derive(Debug, Clone, Copy)]
struct Edge
{
	side: usize,
	facing: Facing,
}

impl Edge
{
	fn encode(self) -> u8
	{
		encode(self.side, self.facing)
	}
}

//
//  01
//  2
// 34
// 5
//
#[rustfmt::skip]
const INPUT_CUBE_CONFIGURATION: [Configuration; NUM_SIDES] = [
	Configuration {
		left: Edge { side: 3, facing: Facing::Right },
		right: Edge { side: 1, facing: Facing::Right },
		up: Edge { side: 5, facing: Facing::Right },
		down: Edge { side: 2, facing: Facing::Down },
	},
	Configuration {
		left: Edge { side: 0, facing: Facing::Left },
		right: Edge { side: 4, facing: Facing::Left },
		up: Edge { side: 5, facing: Facing::Up },
		down: Edge { side: 2, facing: Facing::Left },
	},
	Configuration {
		left: Edge { side: 3, facing: Facing::Down },
		right: Edge { side: 1, facing: Facing::Up },
		up: Edge { side: 0, facing: Facing::Down },
		down: Edge { side: 4, facing: Facing::Down },
	},
	Configuration {
		left: Edge { side: 0, facing: Facing::Right },
		right: Edge { side: 4, facing: Facing::Right },
		up: Edge { side: 2, facing: Facing::Right },
		down: Edge { side: 5, facing: Facing::Down },
	},
	Configuration {
		left: Edge { side: 3, facing: Facing::Left },
		right: Edge { side: 1, facing: Facing::Left },
		up: Edge { side: 2, facing: Facing::Up },
		down: Edge { side: 5, facing: Facing::Left },
	},
	Configuration {
		left: Edge { side: 0, facing: Facing::Down },
		right: Edge { side: 4, facing: Facing::Up },
		up: Edge { side: 3, facing: Facing::Up },
		down: Edge { side: 1, facing: Facing::Down },
	},
];

//
//   0
// 123
//   45
//
#[rustfmt::skip]
const PROVIDED_CUBE_CONFIGURATION: [Configuration; NUM_SIDES] = [
	Configuration {
		left: Edge { side: 2, facing: Facing::Down },
		right: Edge { side: 5, facing: Facing::Left },
		up: Edge { side: 1, facing: Facing::Down },
		down: Edge { side: 3, facing: Facing::Down },
	},
	Configuration {
		left: Edge { side: 5, facing: Facing::Up },
		right: Edge { side: 2, facing: Facing::Right },
		up: Edge { side: 0, facing: Facing::Down },
		down: Edge { side: 4, facing: Facing::Up },
	},
	Configuration {
		left: Edge { side: 1, facing: Facing::Left },
		right: Edge { side: 3, facing: Facing::Right },
		up: Edge { side: 0, facing: Facing::Right },
		down: Edge { side: 4, facing: Facing::Right },
	},
	Configuration {
		left: Edge { side: 2, facing: Facing::Left },
		right: Edge { side: 5, facing: Facing::Down },
		up: Edge { side: 0, facing: Facing::Up },
		down: Edge { side: 4, facing: Facing::Down },
	},
	Configuration {
		left: Edge { side: 2, facing: Facing::Up },
		right: Edge { side: 5, facing: Facing::Right },
		up: Edge { side: 3, facing: Facing::Up },
		down: Edge { side: 1, facing: Facing::Up },
	},
	Configuration {
		left: Edge { side: 4, facing: Facing::Left },
		right: Edge { side: 0, facing: Facing::Left },
		up: Edge { side: 3, facing: Facing::Left },
		down: Edge { side: 1, facing: Facing::Right },
	},
];

#[allow(unused)]
fn dbg_print_sides(
	sides: &[[[u8; GRID_SIZE]; GRID_SIZE]; NUM_SIDES],
	radius: usize,
)
{
	for s in 0..NUM_SIDES
	{
		println!("Side {}:", char::from(b'A' + s as u8));
		for r in 0..=(radius + 1)
		{
			for x in &sides[s][r][0..=(radius + 1)]
			{
				match x
				{
					b'.' => print!(" __"),
					b'#' => print!(" ##"),
					x => print!(" {:02X}", x),
				}
			}
			println!();
		}
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");
	const PROVIDED_GRID: &str = include_str!("provided_grid.txt");
	const PROVIDED_RADIUS: usize = 4;

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED, PROVIDED_RADIUS), 6032);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED, PROVIDED_RADIUS), 5031);
	}

	#[test]
	fn one_samples()
	{
		assert_eq!(one_1(PROVIDED_RADIUS, PROVIDED_GRID, "0"), 1036);
		assert_eq!(one_1(PROVIDED_RADIUS, PROVIDED_GRID, "1R"), 1041);
		assert_eq!(one_1(PROVIDED_RADIUS, PROVIDED_GRID, "2R"), 1045);
		assert_eq!(one_1(PROVIDED_RADIUS, PROVIDED_GRID, "3R"), 1045);
		assert_eq!(one_1(PROVIDED_RADIUS, PROVIDED_GRID, "1L"), 1043);
		assert_eq!(one_1(PROVIDED_RADIUS, PROVIDED_GRID, "2L"), 1047);
		assert_eq!(one_1(PROVIDED_RADIUS, PROVIDED_GRID, "1RR1"), 1038);
	}

	#[test]
	fn one_wrap_around()
	{
		assert_eq!(one_1(PROVIDED_RADIUS, PROVIDED_GRID, "RR1"), 1038);
	}

	#[test]
	fn one_turn_left()
	{
		assert_eq!(one_1(PROVIDED_RADIUS, PROVIDED_GRID, "1L1L1L1L"), 1036);
	}
}
