/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> i32
{
	let mut grid = Grid::parse(input);
	//grid.dbg_print();
	let mut t = 0;
	while !grid.has_reached_exit()
	{
		t += 1;
		grid.step();
		//grid.dbg_print();
	}
	t
}

fn two(_input: &str) -> i32
{
	0
}

#[allow(dead_code)]
const MAX_COLS: usize = 128;
const MAX_ROWS: usize = 32;

struct Grid
{
	walls: [u128; MAX_ROWS],
	wind_north: [u128; MAX_ROWS],
	wind_south: [u128; MAX_ROWS],
	wind_less: [u128; MAX_ROWS],
	wind_more: [u128; MAX_ROWS],
	presence: [u128; MAX_ROWS],
	width: usize,
	height: usize,
}

impl Grid
{
	fn parse(input: &str) -> Grid
	{
		let mut width = 0;
		let mut height = 0;
		let mut walls = [0u128; MAX_ROWS];
		let mut wind_north = [0u128; MAX_ROWS];
		let mut wind_south = [0u128; MAX_ROWS];
		let mut wind_less = [0u128; MAX_ROWS];
		let mut wind_more = [0u128; MAX_ROWS];
		let mut presence = [0u128; MAX_ROWS];
		for line in input.lines()
		{
			assert!(line.is_ascii());
			if width == 0
			{
				width = line.len();
			}
			else
			{
				assert_eq!(line.len(), width);
			}
			for (c, glyph) in line.bytes().enumerate()
			{
				match glyph
				{
					b'#' => walls[height] |= 1 << c,
					b'.' => (),
					b'^' => wind_north[height] |= 1 << c,
					b'v' => wind_south[height] |= 1 << c,
					b'<' => wind_less[height] |= 1 << c,
					b'>' => wind_more[height] |= 1 << c,
					_ => unreachable!(),
				}
			}
			height += 1;
		}
		presence[0] = 1 << 1;
		Grid {
			presence,
			walls,
			wind_north,
			wind_south,
			wind_less,
			wind_more,
			width,
			height,
		}
	}

	fn has_reached_exit(&self) -> bool
	{
		(self.presence[self.height - 1] & (1 << (self.width - 2))) != 0
	}

	fn step(&mut self)
	{
		self.wind_north[1..(self.height - 1)].rotate_left(1);
		self.wind_south[1..(self.height - 1)].rotate_right(1);
		for r in 1..(self.height - 1)
		{
			self.wind_less[r] = blow_l(self.wind_less[r], self.walls[r]);
			self.wind_more[r] = blow_m(self.wind_more[r], self.walls[r]);
		}

		let mut below = 0;
		// We do not need to move back to the starting position, so we can
		// skip updating self.presence[0] and safely use r - 1.
		for r in (1..self.height).rev()
		{
			// We can move in four directions or stay still.
			let current = self.presence[r];
			self.presence[r] |=
				self.presence[r - 1] | (current >> 1) | (current << 1) | below;
			below = current;
			// The example makes it clear that you can brave the storm by
			// moving right through it, as long as you don't end up in one.
			let obstacle =
				self.walls[r]
					| self.wind_north[r] | self.wind_south[r]
					| self.wind_less[r] | self.wind_more[r];
			self.presence[r] &= !obstacle;
		}
	}

	#[allow(dead_code)]
	fn dbg_print(&self)
	{
		println!();
		for r in 0..self.height
		{
			for c in 0..self.width
			{
				let n = (self.wind_north[r] >> c) & 0b1;
				let s = (self.wind_south[r] >> c) & 0b1;
				let l = (self.wind_less[r] >> c) & 0b1;
				let m = (self.wind_more[r] >> c) & 0b1;
				let num = n + s + l + m;
				match num
				{
					0 if ((self.walls[r] >> c) & 0b1) != 0 => print!("#"),
					0 if ((self.presence[r] >> c) & 0b1) != 0 => print!("E"),
					0 => print!("."),
					1 if n > 0 => print!("^"),
					1 if s > 0 => print!("v"),
					1 if l > 0 => print!("<"),
					1 if m > 0 => print!(">"),
					num => print!("{}", num),
				}
			}
			println!();
		}
		println!();
	}
}

fn blow_l(before: u128, walls: u128) -> u128
{
	let wind = before >> 1;
	if wind & walls != 0
	{
		wind | (walls >> 1)
	}
	else
	{
		wind
	}
}

fn blow_m(before: u128, walls: u128) -> u128
{
	let wind = before << 1;
	if wind & walls != 0
	{
		wind | (1 << 1)
	}
	else
	{
		wind
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
		assert_eq!(one(PROVIDED), 18);
	}
}
