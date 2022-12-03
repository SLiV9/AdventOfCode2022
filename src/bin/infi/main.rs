/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> i32
{
	let mut santa = Santa::new();
	let instructions = input.lines().map(|line| line.parse().unwrap());
	for instruction in instructions
	{
		santa.follow(instruction);
	}
	santa.x.abs() + santa.y.abs()
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
enum Instruction
{
	#[display("draai {clockwise_angle}")]
	Turn
	{
		clockwise_angle: i32
	},
	#[display("loop {distance}")]
	Walk
	{
		distance: i32
	},
	#[display("spring {distance}")]
	Jump
	{
		distance: i32
	},
}

#[derive(Debug)]
struct Santa
{
	x: i32,
	y: i32,
	facing_angle: i32,
	dx: i32,
	dy: i32,
	snow: [u8; SNOW_BUFFER_SIZE],
}

//
// 3 2 1
// 4   0
// 5 6 7
//
const DX_PER_QUARTER_PI: [i32; 8] = [1, 1, 0, -1, -1, -1, 0, 1];
const DY_PER_QUARTER_PI: [i32; 8] = [0, -1, -1, -1, 0, 1, 1, 1];

impl Santa
{
	fn new() -> Santa
	{
		let mut santa = Santa {
			x: 0,
			y: 0,
			facing_angle: 90,
			dx: 0,
			dy: 0,
			snow: [0u8; SNOW_BUFFER_SIZE],
		};

		santa.fix_dx_dy();
		santa
	}

	fn follow(&mut self, instruction: Instruction)
	{
		match instruction
		{
			Instruction::Turn { clockwise_angle } =>
			{
				let angle = -clockwise_angle;
				self.facing_angle = (360 + self.facing_angle + angle) % 360;
				self.fix_dx_dy();
			}
			Instruction::Walk { distance } =>
			{
				if distance >= 1
				{
					for i in 1..=distance
					{
						self.step(self.x + i * self.dx, self.y + i * self.dy);
					}
				}
				else
				{
					for i in distance..0
					{
						self.step(self.x + i * self.dx, self.y + i * self.dy);
					}
				}
				self.x += distance * self.dx;
				self.y += distance * self.dy;
			}
			Instruction::Jump { distance } =>
			{
				self.x += distance * self.dx;
				self.y += distance * self.dy;
				self.step(self.x, self.y);
			}
		}
	}

	fn fix_dx_dy(&mut self)
	{
		let quarters_pi = (self.facing_angle / 45) as usize;
		self.dx = DX_PER_QUARTER_PI[quarters_pi];
		self.dy = DY_PER_QUARTER_PI[quarters_pi];
	}

	fn step(&mut self, x: i32, y: i32)
	{
		let xx: usize = (x - MIN_X).try_into().unwrap();
		let yy: usize = (y - MIN_Y).try_into().unwrap();
		let offset = yy * STRIDE + 1 + xx;
		self.snow[offset] = b' ';
	}
}

const MIN_X: i32 = -10;
const MIN_Y: i32 = -10;
const MAX_X: i32 = 80;
const MAX_Y: i32 = 10;
const WIDTH: usize = (MAX_X - MIN_X) as usize;
const HEIGHT: usize = (MAX_Y - MIN_Y) as usize;
const STRIDE: usize = WIDTH + 1;
const SNOW_BUFFER_SIZE: usize = HEIGHT * STRIDE;

fn two(input: &str) -> String
{
	let mut santa = Santa::new();
	santa.snow = [b'#'; SNOW_BUFFER_SIZE];
	for r in 0..HEIGHT
	{
		santa.snow[r * STRIDE] = b'\n';
	}
	let instructions = input.lines().map(|line| line.parse().unwrap());
	for instruction in instructions
	{
		santa.follow(instruction);
	}
	std::str::from_utf8(&santa.snow).unwrap().to_string()
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
		assert_eq!(one(PROVIDED), 12);
	}
}
