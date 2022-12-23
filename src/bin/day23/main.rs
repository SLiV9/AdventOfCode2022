/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

const PROPOSAL_SEQUENCE_LEN: usize = 4;
const PROPOSAL_SEQUENCE: [u8; PROPOSAL_SEQUENCE_LEN] = [b'N', b'S', b'W', b'E'];

fn one(input: &str) -> usize
{
	let mut grid = Grid::create();
	grid.parse_input(input);
	while grid.is_active
	{
		grid.expand();
		grid.diffuse();
		grid.collapse();
	}
	grid.count_empty_spaces()
}

fn two(_input: &str) -> i32
{
	0
}

const MAX_ROWS: usize = 128;
const MAX_COLS: usize = 128;

struct Grid
{
	data: [u128; MAX_ROWS],
	width: usize,
	height: usize,
	proposal_sequence: [u8; PROPOSAL_SEQUENCE_LEN],
	is_active: bool,
}

impl Grid
{
	fn create() -> Grid
	{
		Grid {
			data: [0u128; MAX_ROWS],
			width: 0,
			height: 0,
			proposal_sequence: PROPOSAL_SEQUENCE,
			is_active: true,
		}
	}

	fn parse_input(&mut self, input: &str)
	{
		for (r, line) in input.lines().enumerate()
		{
			for (c, byte) in line.bytes().enumerate()
			{
				match byte
				{
					b'#' => self.set_at_rc(r, c),
					_ => (),
				}
			}
		}
	}

	fn set_at_rc(&mut self, r: usize, c: usize)
	{
		self.data[r] |= 1u128 << c;
		if r > self.height
		{
			self.height = r;
		}
		if c > self.width
		{
			self.width = c;
		}
	}

	fn expand(&mut self)
	{
		// Add an empty row at the top and the bottom.
		self.data[0..(self.height + 1)].rotate_left(self.height);
		self.height += 2;
		assert!(self.height <= MAX_ROWS);
		// Add an empty column at the start and the end.
		for row in &mut self.data
		{
			*row <<= 1;
		}
		self.width += 2;
		assert!(self.width <= MAX_COLS);
	}

	fn diffuse(&mut self)
	{
		self.is_active = false;
		let mut proposed_previous_line: u128 = 0;
		for r in 0..self.height
		{
			// TODO implement some weird scan line algorithm
		}
		self.proposal_sequence.rotate_left(1);
	}

	fn collapse(&mut self)
	{
		// Trim empty rows.
		let from = self.data.iter().position(|&row| row != 0).unwrap();
		let to = self.data.iter().rposition(|&row| row != 0).unwrap();
		self.data[0..to].rotate_left(from);
		self.height = to - from;
		// Trim empty columns.
		let mask = self.data[0..self.height].iter().fold(0, |a, row| (a | row));
		let n = mask.trailing_zeros();
		for row in &mut self.data
		{
			*row >>= n;
		}
		self.width = MAX_COLS - n as usize - mask.leading_zeros() as usize;
	}

	fn count_empty_spaces(&self) -> usize
	{
		self.data[0..self.height]
			.iter()
			.map(|word| self.width - word.count_ones() as usize)
			.sum()
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
		assert_eq!(one(PROVIDED), 110);
	}
}
