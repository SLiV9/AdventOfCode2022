/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> usize
{
	let mut cave = Cave::default();
	run_simulation(&mut cave, input, 2022);
	cave.height_from_floor
}

fn two(input: &str) -> usize
{
	let mut cave = Cave::default();
	run_simulation(&mut cave, input, 1000000000000);
	cave.height_from_floor
}

const NUM_SHAPES: usize = 5;
const SHAPE_HEIGHT: usize = 4;
const SHAPE_WIDTH: [usize; NUM_SHAPES] = [4, 3, 3, 1, 2];
// Shape data is stored with the left-most pixel in least-significant position,
// and the bottom row as 0.
const SHAPE_DATA: [[u8; SHAPE_HEIGHT]; NUM_SHAPES] = [
	// 0000
	// 0000
	// 0000
	// 1111
	[0b1111, 0b0000, 0b0000, 0b0000],
	// 0000
	// 0100
	// 1110
	// 0100
	[0b0010, 0b0111, 0b0010, 0b0000],
	// 0000
	// 0010
	// 0010
	// 1110
	[0b0111, 0b0100, 0b0100, 0b0000],
	// 1000
	// 1000
	// 1000
	// 1000
	[0b0001, 0b0001, 0b0001, 0b0001],
	// 0000
	// 0000
	// 1100
	// 1100
	[0b0011, 0b0011, 0b0000, 0b0000],
];

const CAVE_WIDTH: usize = 7;
const MAX_HEIGHT: usize = 8000;

struct Cave
{
	height_from_floor: usize,
	height_from_cutoff: usize,
	y_of_cutoff: usize,
	grid: [u8; MAX_HEIGHT],
	water: [u8; MAX_HEIGHT],
}

impl Default for Cave
{
	fn default() -> Cave
	{
		Cave {
			height_from_floor: 0,
			height_from_cutoff: 0,
			y_of_cutoff: 0,
			grid: [0; MAX_HEIGHT],
			water: [0; MAX_HEIGHT],
		}
	}
}

impl Cave
{
	fn fits(&self, shape: [u8; SHAPE_HEIGHT], x: usize, y: usize) -> bool
	{
		(0..SHAPE_HEIGHT).all(|i| ((shape[i] << x) & self.grid[y + i]) == 0)
	}

	fn place(&mut self, shape: [u8; SHAPE_HEIGHT], x: usize, y: usize)
	{
		for i in 0..SHAPE_HEIGHT
		{
			let slice = shape[i] << x;
			if slice != 0
			{
				self.grid[y + i] |= slice;
				let h = y + i + 1;
				if h > self.height_from_cutoff
				{
					self.height_from_cutoff = h;
					self.height_from_floor = self.y_of_cutoff + h;
				}
			}
		}
	}

	fn perform_cutoff(&mut self)
	{
		// TODO perform floodfill from the top
		// TODO determine the lowest y reached by the floodfill
		// TODO cutoff at that y
		unimplemented!()
	}
}

fn run_simulation(cave: &mut Cave, input: &str, number_of_rocks: usize)
{
	let wind_data = input.as_bytes();
	let wind_data_len = wind_data.len();
	let mut wind_offset = 0;
	let mut shape_offset = 0;
	for _k in 0..number_of_rocks
	{
		let shape = SHAPE_DATA[shape_offset];
		let shape_width = SHAPE_WIDTH[shape_offset];
		let mut x = 2;
		let mut y = cave.height_from_cutoff + 3;

		if y + SHAPE_HEIGHT >= MAX_HEIGHT
		{
			cave.perform_cutoff();
		}
		assert!(y + SHAPE_HEIGHT < MAX_HEIGHT);

		loop
		{
			let wind = wind_data[wind_offset];
			wind_offset = (wind_offset + 1) % wind_data_len;

			match wind
			{
				b'>' if x + shape_width >= CAVE_WIDTH => (),
				b'>' =>
				{
					if cave.fits(shape, x + 1, y)
					{
						x += 1;
					}
				}
				b'<' if x == 0 => (),
				b'<' =>
				{
					if cave.fits(shape, x - 1, y)
					{
						x -= 1;
					}
				}
				b'=' => (),
				b'\n' => continue,
				_ => unreachable!(),
			}

			if y == 0
			{
				break;
			}

			if !cave.fits(shape, x, y - 1)
			{
				break;
			}

			y -= 1;
		}

		cave.place(shape, x, y);

		shape_offset = (shape_offset + 1) % NUM_SHAPES;
	}
}

#[allow(unused)]
fn print_cave(data: &[u8])
{
	// The cave is stored with the floor at 0.
	for row in data.iter().rev()
	{
		print!("|");
		for x in 0..CAVE_WIDTH
		{
			if (row >> x) & 0b1 != 0
			{
				print!("#");
			}
			else
			{
				print!(" ");
			}
		}
		println!("|");
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
		assert_eq!(one(PROVIDED), 3068);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 1514285714288);
	}

	#[test]
	fn one_without_wind()
	{
		assert_eq!(one("="), 4448);
	}
}
