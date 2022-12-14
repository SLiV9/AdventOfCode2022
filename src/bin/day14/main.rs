/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> usize
{
	let mut cave = Cave::new();
	cave.build(input);
	cave.fill();
	cave.count_sand()
}

fn two(_input: &str) -> i32
{
	0
}

const WIDTH: usize = 1000;
const HEIGHT: usize = 200;

#[derive(Debug)]
struct Cave
{
	// As a micro-optimization, use column-major order because it might make
	// dropping sand multiple tiles slightly faster.
	grid_xy: [[u8; HEIGHT]; WIDTH],
}

impl Cave
{
	fn new() -> Cave
	{
		Cave {
			grid_xy: [[b' '; HEIGHT]; WIDTH],
		}
	}

	fn build(&mut self, input: &str)
	{
		for line in input.lines()
		{
			let mut coordinates =
				line.split(" -> ").map(|pos| pos.parse().unwrap());
			if let Some(head) = coordinates.next()
			{
				let mut from: Position = head;
				for to in coordinates
				{
					if from.y == to.y
					{
						let y = from.y;
						let x0 = std::cmp::min(from.x, to.x);
						let x1 = std::cmp::max(from.x, to.x);
						assert!(x0 > 0);
						assert!(x1 + 1 < WIDTH);
						assert!(y > 0);
						assert!(y + 1 < HEIGHT);
						for x in x0..=x1
						{
							self.grid_xy[x][y] = b'#';
						}
					}
					else if from.x == to.x
					{
						let x = from.x;
						let y0 = std::cmp::min(from.y, to.y);
						let y1 = std::cmp::max(from.y, to.y);
						assert!(x > 0);
						assert!(x + 1 < WIDTH);
						assert!(y0 > 0);
						assert!(y1 + 1 < HEIGHT);
						for y in y0..=y1
						{
							self.grid_xy[x][y] = b'#';
						}
					}
					else
					{
						panic!("diagonal");
					}
					from = to;
				}
			}
		}
	}

	fn fill(&mut self)
	{
		while let Some(final_pos) = self.drop_grain()
		{
			self.grid_xy[final_pos.x][final_pos.y] = b'o';
		}
	}

	fn drop_grain(&mut self) -> Option<Position>
	{
		let mut pos = Position { x: 500, y: 0 };
		while pos.y + 1 < HEIGHT
		{
			//dbg!(&pos);
			if self.grid_xy[pos.x][pos.y + 1] == b' '
			{
				pos.y += 1;
			}
			else if self.grid_xy[pos.x - 1][pos.y + 1] == b' '
			{
				pos.x -= 1;
				pos.y += 1;
			}
			else if self.grid_xy[pos.x + 1][pos.y + 1] == b' '
			{
				pos.x += 1;
				pos.y += 1;
			}
			else
			{
				return Some(pos);
			}
		}
		return None;
	}

	fn count_sand(&self) -> usize
	{
		self.grid_xy
			.iter()
			.map(|row| row.iter().filter(|a| **a == b'o').count())
			.sum()
	}
}

#[derive(
	Debug, Clone, Copy, parse_display::Display, parse_display::FromStr,
)]
#[display("{x},{y}")]
struct Position
{
	x: usize,
	y: usize,
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
		assert_eq!(one(PROVIDED), 24);
	}
}
