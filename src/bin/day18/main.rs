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

fn two(_input: &str) -> i32
{
	0
}

#[derive(Debug, Clone, Copy)]
#[derive(parse_display::Display, parse_display::FromStr)]
#[display("{x},{y},{z}")]
struct Pixel
{
	x: i8,
	y: i8,
	z: i8,
}

const MAX_DIMENSION: usize = 128;

#[derive(Debug)]
struct Sides
{
	x_planes: [[u128; MAX_DIMENSION]; MAX_DIMENSION],
	y_planes: [[u128; MAX_DIMENSION]; MAX_DIMENSION],
	z_planes: [[u128; MAX_DIMENSION]; MAX_DIMENSION],
}

impl Default for Sides
{
	fn default() -> Sides
	{
		Sides {
			x_planes: [[0; MAX_DIMENSION]; MAX_DIMENSION],
			y_planes: [[0; MAX_DIMENSION]; MAX_DIMENSION],
			z_planes: [[0; MAX_DIMENSION]; MAX_DIMENSION],
		}
	}
}

impl Sides
{
	fn insert(&mut self, p: Pixel)
	{
		self.x_planes[p.x as usize][p.y as usize] ^= 1u128 << p.z;
		self.x_planes[p.x as usize + 1][p.y as usize] ^= 1u128 << p.z;
		self.y_planes[p.y as usize][p.z as usize] ^= 1u128 << p.x;
		self.y_planes[p.y as usize + 1][p.z as usize] ^= 1u128 << p.x;
		self.z_planes[p.z as usize][p.x as usize] ^= 1u128 << p.y;
		self.z_planes[p.z as usize + 1][p.x as usize] ^= 1u128 << p.y;
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

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 64);
	}
}
