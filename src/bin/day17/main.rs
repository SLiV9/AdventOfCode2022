/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
enum Vein
{
	#[display("x={x}, y={y0}..{y1}")]
	Horiz
	{
		x: i32, y0: i32, y1: i32
	},
	#[display("y={y}, x={x0}..{x1}")]
	Vert
	{
		y: i32, x0: i32, x1: i32
	},
}

const WIDTH: usize = 1000;
const MAX_HEIGHT: usize = 1000;

#[derive(Debug)]
struct Board
{
	cells: [[u8; WIDTH]; MAX_HEIGHT],
	y_min: i32,
	y_max: i32,
	x_min: i32,
	x_max: i32,
}

impl Board
{
	fn new() -> Self
	{
		Self {
			cells: [[b'.'; WIDTH]; MAX_HEIGHT],
			y_min: 999,
			y_max: 0,
			x_min: 500,
			x_max: 500,
		}
	}
}

fn build_and_settle(board: &mut Board, input: &str)
{
	for vein in input.lines().map(|line| line.parse().unwrap())
	{
		println!("{}", vein);
		match vein
		{
			Vein::Horiz { .. } => (),
			Vein::Vert { .. } => (),
		}
		// TODO
	}
}

fn one(input: &str) -> i32
{
	let mut board = Board::new();
	build_and_settle(&mut board, input);
	0
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

	#[test]
	fn one_provided()
	{
		assert_eq!(one(include_str!("provided.txt")), 57);
	}
}
