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
	let mut view = vec![-1i8; width];
	let mut height = 0;
	for (row, line) in input.lines().enumerate()
	{
		let mut max = -1;
		for (col, x) in line.as_bytes().iter().enumerate()
		{
			let h: i8 = (x - b'0').try_into().unwrap();
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
	view.fill(-1);
	for (rev_row, line) in input.lines().rev().enumerate()
	{
		let row = height - 1 - rev_row;
		let mut max = -1;
		for (rev_col, x) in line.as_bytes().iter().rev().enumerate()
		{
			let col = width - 1 - rev_col;
			let h: i8 = (x - b'0').try_into().unwrap();
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

fn two(_input: &str) -> usize
{
	0
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
}
