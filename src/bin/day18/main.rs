/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> u32
{
	let mut sides = Sides::default();
	for pixel in input.lines().map(|line| line.parse().unwrap())
	{
		sides.insert(pixel);
	}
	sides.count()
}

fn two(input: &str) -> u32
{
	let mut sides = Sides::default();
	for pixel in input.lines().map(|line| line.parse().unwrap())
	{
		sides.insert(pixel);
	}
	let total = sides.count();
	sides.erase_exterior();
	total - sides.count()
}

#[derive(Debug, Default, Clone, Copy)]
#[derive(parse_display::Display, parse_display::FromStr)]
#[display("{x},{y},{z}")]
struct Pixel
{
	x: i8,
	y: i8,
	z: i8,
}

const MAX_DIMENSION: usize = 128;
const MAX_STACK_SIZE: usize = MAX_DIMENSION * MAX_DIMENSION;

#[derive(Debug)]
struct Sides
{
	x_planes: [[u128; MAX_DIMENSION]; MAX_DIMENSION],
	y_planes: [[u128; MAX_DIMENSION]; MAX_DIMENSION],
	z_planes: [[u128; MAX_DIMENSION]; MAX_DIMENSION],
	max_x: i8,
	max_y: i8,
	max_z: i8,
}

impl Default for Sides
{
	fn default() -> Sides
	{
		Sides {
			x_planes: [[0; MAX_DIMENSION]; MAX_DIMENSION],
			y_planes: [[0; MAX_DIMENSION]; MAX_DIMENSION],
			z_planes: [[0; MAX_DIMENSION]; MAX_DIMENSION],
			max_x: 0,
			max_y: 0,
			max_z: 0,
		}
	}
}

impl Sides
{
	fn insert(&mut self, p: Pixel)
	{
		if p.x > self.max_x
		{
			self.max_x = p.x;
		}
		if p.y > self.max_y
		{
			self.max_y = p.y;
		}
		if p.z > self.max_z
		{
			self.max_z = p.z;
		}
		self.x_planes[p.x as usize][p.y as usize] ^= 1u128 << p.z;
		self.x_planes[p.x as usize + 1][p.y as usize] ^= 1u128 << p.z;
		self.y_planes[p.y as usize][p.z as usize] ^= 1u128 << p.x;
		self.y_planes[p.y as usize + 1][p.z as usize] ^= 1u128 << p.x;
		self.z_planes[p.z as usize][p.x as usize] ^= 1u128 << p.y;
		self.z_planes[p.z as usize + 1][p.x as usize] ^= 1u128 << p.y;
	}

	fn erase(&mut self, p: Pixel)
	{
		self.x_planes[p.x as usize][p.y as usize] &= !(1u128 << p.z);
		self.x_planes[p.x as usize + 1][p.y as usize] &= !(1u128 << p.z);
		self.y_planes[p.y as usize][p.z as usize] &= !(1u128 << p.x);
		self.y_planes[p.y as usize + 1][p.z as usize] &= !(1u128 << p.x);
		self.z_planes[p.z as usize][p.x as usize] &= !(1u128 << p.y);
		self.z_planes[p.z as usize + 1][p.x as usize] &= !(1u128 << p.y);
	}

	fn erase_exterior(&mut self)
	{
		let mut steam = Steam::default();
		// Surround the box with steam.
		dbg!(self.max_x);
		dbg!(self.max_y);
		dbg!(self.max_z);
		for x in [0, self.max_x + 1]
		{
			for y in 0..=self.max_y
			{
				let line = self.x_planes[x as usize][y as usize];
				for z in 0..=self.max_z
				{
					if x > 0 || (line & (1u128 << z)) == 0
					{
						steam.push(Pixel { x, y, z });
					}
				}
			}
		}
		for y in [0, self.max_y + 1]
		{
			for z in 0..=self.max_z
			{
				let line = self.y_planes[y as usize][z as usize];
				for x in 0..=self.max_x
				{
					if y > 0 || (line & (1u128 << x)) == 0
					{
						steam.push(Pixel { x, y, z });
					}
				}
			}
		}
		for z in [0, self.max_z + 1]
		{
			for x in 0..=self.max_x
			{
				let line = self.z_planes[z as usize][x as usize];
				for y in 0..=self.max_y
				{
					if z > 0 || (line & (1u128 << y)) == 0
					{
						steam.push(Pixel { x, y, z });
					}
				}
			}
		}
		// Do a floodfill with steam.
		dbg!(steam.stack_size);
		while steam.stack_size > 0
		{
			steam.stack_size -= 1;
			let from: Pixel = steam.stack[steam.stack_size];
			expand(self, &mut steam, from);
		}
		// Erase all the sides that touch steam.
		self.x_planes[0].fill(0);
		self.y_planes[0].fill(0);
		self.z_planes[0].fill(0);
		for x in 0..=(self.max_x + 1)
		{
			for y in 0..=(self.max_y + 1)
			{
				let line = steam.xyz[x as usize][y as usize];
				for z in 0..=(self.max_z + 1)
				{
					if (line & (1u128 << z)) != 0
					{
						self.erase(Pixel { x, y, z });
					}
				}
			}
		}
	}

	fn count(&self) -> u32
	{
		count_planes(&self.x_planes)
			+ count_planes(&self.y_planes)
			+ count_planes(&self.z_planes)
	}
}

fn count_planes(planes: &[[u128; MAX_DIMENSION]; MAX_DIMENSION]) -> u32
{
	planes.iter().map(count_plane).sum()
}

fn count_plane(plane: &[u128; MAX_DIMENSION]) -> u32
{
	plane.iter().map(|line| line.count_ones()).sum()
}

#[derive(Debug)]
struct Steam
{
	xyz: [[u128; MAX_DIMENSION]; MAX_DIMENSION],
	stack: [Pixel; MAX_STACK_SIZE],
	stack_size: usize,
}

impl Default for Steam
{
	fn default() -> Steam
	{
		Steam {
			xyz: [[0u128; MAX_DIMENSION]; MAX_DIMENSION],
			stack: [Pixel::default(); MAX_STACK_SIZE],
			stack_size: 0,
		}
	}
}

impl Steam
{
	fn push(&mut self, p: Pixel)
	{
		if (self.xyz[p.x as usize][p.y as usize] & (1u128 << p.z)) != 0
		{
			return;
		}
		self.xyz[p.x as usize][p.y as usize] |= 1u128 << p.z;
		self.stack[self.stack_size] = p;
		self.stack_size += 1;
	}
}

fn expand(data: &Sides, steam: &mut Steam, p: Pixel)
{
	if p.x > 1
		&& (data.x_planes[p.x as usize][p.y as usize] & (1u128 << p.z)) == 0
	{
		steam.push(Pixel { x: p.x - 1, ..p });
	}
	if p.x < data.max_x
		&& (data.x_planes[p.x as usize + 1][p.y as usize] & (1u128 << p.z)) == 0
	{
		steam.push(Pixel { x: p.x + 1, ..p });
	}
	if p.y > 1
		&& (data.y_planes[p.y as usize][p.z as usize] & (1u128 << p.x)) == 0
	{
		steam.push(Pixel { y: p.y - 1, ..p });
	}
	if p.y < data.max_y
		&& (data.y_planes[p.y as usize + 1][p.z as usize] & (1u128 << p.x)) == 0
	{
		steam.push(Pixel { y: p.y + 1, ..p });
	}
	if p.z > 1
		&& (data.z_planes[p.z as usize][p.x as usize] & (1u128 << p.y)) == 0
	{
		steam.push(Pixel { z: p.z - 1, ..p });
	}
	if p.z < data.max_z
		&& (data.z_planes[p.z as usize + 1][p.x as usize] & (1u128 << p.y)) == 0
	{
		steam.push(Pixel { z: p.z + 1, ..p });
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");
	const CUBE_2X2X2: &str = include_str!("cube_2x2x2.txt");
	const CUBE_3X3X3: &str = include_str!("cube_3x3x3.txt");
	const CUBE_WITH_HOLE: &str = include_str!("cube_with_hole.txt");
	const CATBERT: &str = include_str!("catbert.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 64);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 58);
	}

	#[test]
	fn one_cube_pixel()
	{
		assert_eq!(one("0,0,0"), 6);
	}

	#[test]
	fn two_cube_pixel()
	{
		assert_eq!(two("0,0,0"), 6);
	}

	#[test]
	fn one_cube_1x1x1()
	{
		assert_eq!(one("1,1,1"), 6);
	}

	#[test]
	fn two_cube_1x1x1()
	{
		assert_eq!(two("1,1,1"), 6);
	}

	#[test]
	fn one_cube_2x2x2()
	{
		assert_eq!(one(CUBE_2X2X2), 24);
	}

	#[test]
	fn two_cube_2x2x2()
	{
		assert_eq!(two(CUBE_2X2X2), 24);
	}

	#[test]
	fn one_cube_3x3x3()
	{
		assert_eq!(one(CUBE_3X3X3), 54);
	}

	#[test]
	fn two_cube_3x3x3()
	{
		assert_eq!(two(CUBE_3X3X3), 54);
	}

	#[test]
	fn one_cube_with_hole()
	{
		assert_eq!(one(CUBE_WITH_HOLE), 60);
	}

	#[test]
	fn two_cube_with_hole()
	{
		assert_eq!(two(CUBE_WITH_HOLE), 54);
	}

	#[test]
	fn one_catbert()
	{
		assert_eq!(one(CATBERT), 108);
	}

	#[test]
	fn two_catbert()
	{
		assert_eq!(two(CATBERT), 90);
	}
}
