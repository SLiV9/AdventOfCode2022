/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

// Instead of east and west, use LESS significant and MORE significant.
const PROPOSAL_SEQUENCE_LEN: usize = 4;
const PROPOSAL_SEQUENCE: [u8; PROPOSAL_SEQUENCE_LEN] = [b'N', b'S', b'L', b'M'];

fn one(input: &str) -> usize
{
	let mut grid = Grid::create();
	grid.parse_input(input);
	grid.dbg_print();
	let count = grid.count();
	for round in 1..=10
	{
		//println!("Round {}", round);
		let _ = round;
		grid.expand();
		grid.diffuse();
		//grid.dbg_print();
		assert_eq!(grid.count(), count);
		if !grid.is_active
		{
			break;
		}
	}
	grid.collapse();
	assert_eq!(grid.count(), count);
	grid.dbg_print();
	grid.count_empty_spaces()
}

fn two(input: &str) -> usize
{
	let mut grid = Grid::create();
	grid.parse_input(input);
	let count = grid.count();
	let mut round = 0;
	while grid.is_active
	{
		round += 1;
		//println!("Round {}", round);
		grid.expand();
		//grid.dbg_print();
		grid.diffuse();
		//grid.dbg_print();
		assert_eq!(grid.count(), count);
	}
	grid.collapse();
	assert_eq!(grid.count(), count);
	grid.dbg_print();
	round
}

fn propose(
	current: Row,
	above: Row,
	below: Row,
	proposal_sequence: &[u8; PROPOSAL_SEQUENCE_LEN],
) -> Proposal
{
	if current.is_empty()
	{
		return Proposal::default();
	}
	let l = current << 1;
	let m = current >> 1;
	let above3 = above | (above >> 1) | (above << 1);
	let below3 = below | (below >> 1) | (below << 1);
	let happy = current & !l & !m & !above3 & !below3;
	let mut unhappy = current & !happy;
	let mut proposal = Proposal::default();
	for proposed_direction in proposal_sequence.iter().map(|x| *x)
	{
		match proposed_direction
		{
			b'N' =>
			{
				proposal.north = unhappy & !above3;
				unhappy &= !proposal.north;
			}
			b'S' =>
			{
				proposal.south = unhappy & !below3;
				unhappy &= !proposal.south;
			}
			b'L' =>
			{
				let from = unhappy & !l & !(above << 1) & !(below << 1);
				proposal.less = from >> 1;
				unhappy &= !from;
			}
			b'M' =>
			{
				let from = unhappy & !m & !(above >> 1) & !(below >> 1);
				proposal.more = from << 1;
				unhappy &= !from;
			}
			_ => unreachable!(),
		}
	}
	proposal.stay = happy | unhappy;
	proposal
}

#[derive(Debug, Default, Clone)]
struct Proposal
{
	stay: Row,
	north: Row,
	south: Row,
	less: Row,
	more: Row,
}

fn block(a: Row, b: Row, c: Row, d: Row) -> Row
{
	(a & (b | c | d)) | (b & (c | d)) | (c & d)
}

fn resolve_block(blocked: Row, proposed: &mut Row, backup: &mut Row)
{
	let canceled = blocked & *proposed;
	*proposed &= !canceled;
	*backup |= canceled;
}

fn resolve_block_l(blocked: Row, proposed: &mut Row, backup: &mut Row)
{
	let canceled = blocked & *proposed;
	*proposed &= !canceled;
	*backup |= canceled << 1;
}

fn resolve_block_m(blocked: Row, proposed: &mut Row, backup: &mut Row)
{
	let canceled = blocked & *proposed;
	*proposed &= !canceled;
	*backup |= canceled >> 1;
}

#[derive(Debug, Default, Clone, Copy)]
struct Row([u128; MAX_WORDS]);

impl Row
{
	fn is_empty(&self) -> bool
	{
		self.0.iter().all(|&word| word == 0)
	}

	fn set_bit(&mut self, offset: usize)
	{
		self.0[offset / WIDTH_OF_WORD] |= 1 << (offset % WIDTH_OF_WORD);
	}

	fn get_bit(&self, offset: usize) -> bool
	{
		(self.0[offset / WIDTH_OF_WORD] & (1 << (offset % WIDTH_OF_WORD))) != 0
	}

	fn count_ones(&self) -> u32
	{
		self.0.iter().map(|word| word.count_ones()).sum()
	}

	fn leading_zeros(&self) -> u32
	{
		if let Some(k) = self.0.iter().rposition(|&word| word != 0)
		{
			((MAX_WORDS - 1 - k) * WIDTH_OF_WORD) as u32
				+ self.0[k].leading_zeros()
		}
		else
		{
			(MAX_WORDS * WIDTH_OF_WORD) as u32
		}
	}

	fn trailing_zeros(&self) -> u32
	{
		if let Some(k) = self.0.iter().position(|&word| word != 0)
		{
			(k * WIDTH_OF_WORD) as u32 + self.0[k].trailing_zeros()
		}
		else
		{
			(MAX_WORDS * WIDTH_OF_WORD) as u32
		}
	}

	#[allow(unused)]
	fn dbg_print(&self, width: usize)
	{
		print!("| ");
		for c in 0..width
		{
			if self.get_bit(c)
			{
				print!("#");
			}
			else
			{
				print!(".");
			}
		}
		println!();
	}
}

impl std::ops::Not for Row
{
	type Output = Row;

	fn not(mut self) -> Row
	{
		for word in &mut self.0
		{
			*word = !*word;
		}
		self
	}
}

impl std::ops::BitOr for Row
{
	type Output = Row;

	fn bitor(mut self, rhs: Row) -> Row
	{
		self |= rhs;
		self
	}
}

impl std::ops::BitAnd for Row
{
	type Output = Row;

	fn bitand(mut self, rhs: Row) -> Row
	{
		self &= rhs;
		self
	}
}

impl std::ops::BitOrAssign for Row
{
	fn bitor_assign(&mut self, rhs: Row)
	{
		for (a, b) in self.0.iter_mut().zip(rhs.0.iter())
		{
			*a |= b;
		}
	}
}

impl std::ops::BitAndAssign for Row
{
	fn bitand_assign(&mut self, rhs: Row)
	{
		for (a, b) in self.0.iter_mut().zip(rhs.0.iter())
		{
			*a &= b;
		}
	}
}

impl std::ops::Shl<usize> for Row
{
	type Output = Row;

	fn shl(mut self, n: usize) -> Row
	{
		if n >= WIDTH_OF_WORD
		{
			let k = n / WIDTH_OF_WORD;
			self.0.rotate_right(k);
			self.0[0..k].fill(0);
		}
		let n = n % WIDTH_OF_WORD;
		if n > 0
		{
			let mask = ((1u128 << (n + 1)) - 1) << (WIDTH_OF_WORD - n);
			for i in (1..MAX_WORDS).rev()
			{
				let bits = self.0[i - 1] & mask;
				self.0[i] <<= n;
				self.0[i] |= bits >> (WIDTH_OF_WORD - n);
			}
			self.0[0] <<= n;
		}
		self
	}
}

impl std::ops::Shr<usize> for Row
{
	type Output = Row;

	fn shr(mut self, n: usize) -> Row
	{
		if n >= WIDTH_OF_WORD
		{
			let k = n / WIDTH_OF_WORD;
			self.0[0..k].fill(0);
			self.0.rotate_left(k);
		}
		let n = n % WIDTH_OF_WORD;
		if n > 0
		{
			let mask = (1u128 << (n + 1)) - 1;
			self.0[0] >>= n;
			for i in 1..MAX_WORDS
			{
				let bits = self.0[i] & mask;
				self.0[i - 1] |= bits << (WIDTH_OF_WORD - n);
				self.0[i] >>= n;
			}
		}
		self
	}
}

const WIDTH_OF_WORD: usize = 128;
const MAX_WORDS: usize = 4;
const MAX_COLS: usize = WIDTH_OF_WORD * MAX_WORDS;
const MAX_ROWS: usize = MAX_COLS + 1;

struct Grid
{
	data: [Row; MAX_ROWS],
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
			data: [Row::default(); MAX_ROWS],
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
		self.data[r].set_bit(c);
		if r + 1 > self.height
		{
			self.height = r + 1;
		}
		if c + 1 > self.width
		{
			self.width = c + 1;
		}
	}

	fn expand(&mut self)
	{
		// Make sure there is an empty column at the start.
		if self.data[0..self.height]
			.iter()
			.any(|row| row.trailing_zeros() == 0)
		{
			for row in &mut self.data
			{
				*row = *row << 1;
			}
			self.width += 1;
		}
		assert!(self.width <= MAX_COLS);
		// Add an empty row at the bottom and two at the top, if needed.
		if !self.data[0].is_empty()
		{
			self.data[0..(self.height + 1)].rotate_left(self.height);
			self.height += 1;
		}
		if !self.data[self.height - 1].is_empty()
		{
			self.height += 1;
		}
		if !self.data[self.height - 2].is_empty()
		{
			self.height += 1;
		}
		assert!(self.height <= MAX_ROWS);
	}

	fn diffuse(&mut self)
	{
		self.is_active = false;
		assert!(self.height >= 3);
		let mut prev = Proposal::default();
		let mut curr = propose(
			self.data[1],
			self.data[0],
			self.data[2],
			&self.proposal_sequence,
		);
		self.data[0] = curr.north;
		for r_of_prev in 0..(self.height - 3)
		{
			let r_of_current = r_of_prev + 1;
			let r_of_next = r_of_prev + 2;
			let mut next = propose(
				self.data[r_of_next],
				self.data[r_of_next - 1],
				self.data[r_of_next + 1],
				&self.proposal_sequence,
			);
			let blocked = block(prev.south, curr.less, curr.more, next.north);
			resolve_block(blocked, &mut prev.south, &mut self.data[r_of_prev]);
			resolve_block_l(blocked, &mut curr.less, &mut curr.stay);
			resolve_block_m(blocked, &mut curr.more, &mut curr.stay);
			resolve_block(blocked, &mut next.north, &mut next.stay);
			let arrived = prev.south | curr.less | curr.more | next.north;
			if !arrived.is_empty()
			{
				self.is_active = true;
			}
			self.data[r_of_current] = curr.stay | arrived;
			prev = curr;
			curr = next;
		}
		self.data[self.height - 2] |= prev.south;
		assert!(curr.less.is_empty());
		assert!(curr.stay.is_empty());
		assert!(curr.more.is_empty());
		self.proposal_sequence.rotate_left(1);
	}

	fn collapse(&mut self)
	{
		// Trim empty rows.
		let start = self.data.iter().position(|x| !x.is_empty()).unwrap();
		let end = self.data.iter().rposition(|x| !x.is_empty()).unwrap() + 1;
		self.data[0..end].rotate_left(start);
		self.height = end - start;
		// Trim empty columns.
		let mask = self.data[0..self.height]
			.iter()
			.fold(Row::default(), |a, &row| (a | row));
		let n = mask.trailing_zeros() as usize;
		for row in &mut self.data
		{
			*row = (*row) >> n;
		}
		self.width = MAX_COLS - n as usize - mask.leading_zeros() as usize;
	}

	fn count(&self) -> usize
	{
		self.data[0..self.height]
			.iter()
			.map(|row| row.count_ones() as usize)
			.sum()
	}

	fn count_empty_spaces(&self) -> usize
	{
		self.width * self.height - self.count()
	}

	#[allow(unused)]
	fn dbg_print(&self)
	{
		println!();
		for row in &self.data[0..self.height]
		{
			row.dbg_print(self.width);
		}
		println!();
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");
	const MINI: &str = include_str!("mini.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 110);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 20);
	}

	#[test]
	fn one_mini()
	{
		assert_eq!(one(MINI), 25);
	}

	#[test]
	fn one_assertions()
	{
		let _ = one("###\n###\n###\n");
		let _ = one("###\n#.#\n###\n");
		let _ = one("##.##\n##.##\n##.##\n");
		let _ = one(&INPUT[0..200]);
	}

	#[test]
	fn row_assertions()
	{
		let mut row = Row::default();
		let max = MAX_COLS as u32;
		assert_eq!(row.count_ones(), 0);
		assert_eq!(row.leading_zeros(), max);
		assert_eq!(row.trailing_zeros(), max);
		row.set_bit(4);
		assert_eq!(row.count_ones(), 1);
		assert_eq!(row.leading_zeros(), max - 4 - 1);
		assert_eq!(row.trailing_zeros(), 4);
		row.set_bit(200);
		assert_eq!(row.count_ones(), 2);
		assert_eq!(row.leading_zeros(), max - 200 - 1);
		assert_eq!(row.trailing_zeros(), 4);
		row.set_bit(126);
		row.set_bit(127);
		row.set_bit(128);
		row.set_bit(129);
		assert_eq!(row.count_ones(), 6);
		assert_eq!(row.leading_zeros(), max - 200 - 1);
		assert_eq!(row.trailing_zeros(), 4);
		row = row >> 1;
		assert_eq!(row.count_ones(), 6);
		assert_eq!(row.leading_zeros(), max - 200);
		assert_eq!(row.trailing_zeros(), 4 - 1);
		row = row << 2;
		assert_eq!(row.count_ones(), 6);
		assert_eq!(row.leading_zeros(), max - 200 - 2);
		assert_eq!(row.trailing_zeros(), 4 + 1);
		row = row << WIDTH_OF_WORD;
		assert_eq!(row.count_ones(), 6);
		assert_eq!(row.leading_zeros(), max - 200 - 2 - WIDTH_OF_WORD as u32);
		assert_eq!(row.trailing_zeros(), 4 + 1 + WIDTH_OF_WORD as u32);
		row = row >> (WIDTH_OF_WORD + 1);
		assert_eq!(row.count_ones(), 6);
		assert_eq!(row.leading_zeros(), max - 200 - 1);
		assert_eq!(row.trailing_zeros(), 4);
	}
}
