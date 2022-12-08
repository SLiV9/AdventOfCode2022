/**/

const INPUT: &str = include_str!("input.txt");
const INPUT_LEN: usize = INPUT.len();
const GRID_BUFFER_SIZE: usize = (INPUT_LEN + 127) / 128;

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> usize
{
	let width = input.lines().next().unwrap().len();
	let mut grid = Grid::create(width);
	let mut view = vec![0u8; width];
	let mut height = 0;
	for (row, line) in input.lines().enumerate()
	{
		let mut max = 0u8;
		for (col, x) in line.as_bytes().iter().enumerate()
		{
			let h = *x;
			if h > max
			{
				grid.set_at_rc(row, col);
				max = h;
			}
			if h > view[col]
			{
				grid.set_at_rc(row, col);
				view[col] = h;
			}
		}
		height += 1;
	}
	view.fill(0u8);
	for (rev_row, line) in input.lines().rev().enumerate()
	{
		let row = height - 1 - rev_row;
		let mut max = 0u8;
		for (rev_col, x) in line.as_bytes().iter().rev().enumerate()
		{
			let col = width - 1 - rev_col;
			let h = *x;
			if h > max
			{
				grid.set_at_rc(row, col);
				max = h;
			}
			if h > view[col]
			{
				grid.set_at_rc(row, col);
				view[col] = h;
			}
		}
	}
	grid.count()
}

fn two(input: &str) -> usize
{
	let width = input.lines().next().unwrap().len();
	let height = input.lines().count();
	let stride = width + 1;
	let grid = &input.as_bytes();
	let mut max = 0;
	for row in 0..height
	{
		for col in 0..width
		{
			let x = grid[row * stride + col];
			let mut left = 0;
			for c in (0..col).rev()
			{
				let y = grid[row * stride + c];
				left += 1;
				if y >= x
				{
					break;
				}
			}
			let mut right = 0;
			for c in (col + 1)..width
			{
				let y = grid[row * stride + c];
				right += 1;
				if y >= x
				{
					break;
				}
			}
			let mut up = 0;
			for r in (0..row).rev()
			{
				let y = grid[r * stride + col];
				up += 1;
				if y >= x
				{
					break;
				}
			}
			let mut down = 0;
			for r in (row + 1)..height
			{
				let y = grid[r * stride + col];
				down += 1;
				if y >= x
				{
					break;
				}
			}
			let score = left * right * up * down;
			if score > max
			{
				max = score;
			}
		}
	}
	max
}

struct Grid
{
	data: [u128; GRID_BUFFER_SIZE],
	width: usize,
}

impl Grid
{
	fn create(width: usize) -> Grid
	{
		Grid {
			data: [0u128; GRID_BUFFER_SIZE],
			width,
		}
	}

	fn set_at_rc(&mut self, r: usize, c: usize)
	{
		let i = r * self.width + c;
		self.data[i / 128] |= 1 << (i % 128);
	}

	fn count(&self) -> usize
	{
		let sum: u32 = self.data.iter().map(|word| word.count_ones()).sum();
		sum as usize
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
		assert_eq!(one(PROVIDED), 21);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 8);
	}
}
