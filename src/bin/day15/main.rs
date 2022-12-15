/**/

const INPUT: &str = include_str!("input.txt");

const REAL_SCALE: i32 = 4000000;

pub fn main()
{
	println!("Part One: {}", one(INPUT, REAL_SCALE));
	println!("Part Two: {}", two(INPUT, REAL_SCALE));
}

fn one(input: &str, height: i32) -> usize
{
	let scanline_y = height / 2;
	// Collect ranges where the sensor range overlaps the scan line.
	let mut scan_ranges: Vec<Range> = input
		.lines()
		.map(|line| line.parse().unwrap())
		.flat_map(|reading| get_scan_intersection(&reading, scanline_y))
		.filter_map(|x| x)
		.collect();
	// Sort the ranges so that overlapping ranges are adjacent.
	scan_ranges.sort_by_key(|range| *range.start());
	// Count the number of unoccupied spaces after merging overlapping ranges.
	count_deduplicated_ranges(scan_ranges)
}

fn two(input: &str, height: i32) -> u64
{
	let sensors: Vec<Diamond> = input
		.lines()
		.map(|line| line.parse().unwrap())
		.map(|reading: Reading| reading.into())
		.collect();
	let signal = do_edge_search(height, &sensors);
	dbg!(signal);
	(signal.x as u64) * (REAL_SCALE as u64) + (signal.y as u64)
}

fn do_edge_search(scope: i32, sensors: &[Diamond]) -> Position
{
	// We know that there is exactly one solution, so it must be on the outer
	// edge of the range of two or more sensors.
	for traversed in sensors
	{
		// Traverse the outer edge of this diamond.
		//   2
		//  1x3
		// 0xxx7
		//  4x6
		//   5
		let n = traversed.range + 1;
		let nw_edge = (0..n).map(|i| Position {
			x: traversed.center.x - n + i,
			y: traversed.center.y - i,
		});
		let ne_edge = (0..n).map(|i| Position {
			x: traversed.center.x + i,
			y: traversed.center.y - n + i,
		});
		let sw_edge = (0..n).map(|i| Position {
			x: traversed.center.x - n + 1 + i,
			y: traversed.center.y + 1 + i,
		});
		let se_edge = (0..n).map(|i| Position {
			x: traversed.center.x + 1 + i,
			y: traversed.center.y + n - 1 - i,
		});
		let outer_edge = nw_edge.chain(ne_edge).chain(sw_edge).chain(se_edge);
		for pos in outer_edge
		{
			if pos.x < 0 || pos.x > scope || pos.y < 0 || pos.y > scope
			{
				continue;
			}
			if !sensors.iter().any(|sensor| sensor.contains(pos))
			{
				return pos;
			}
		}
	}
	unreachable!()
}

#[derive(Debug, Clone, Copy)]
struct Diamond
{
	center: Position,
	range: i32,
}

impl Diamond
{
	fn contains(&self, pos: Position) -> bool
	{
		manhattan_distance(self.center, pos) <= self.range
	}
}

impl From<Reading> for Diamond
{
	fn from(reading: Reading) -> Diamond
	{
		let range = manhattan_distance(reading.sensor, reading.beacon);
		Diamond {
			center: reading.sensor,
			range,
		}
	}
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[display("Sensor at {sensor}: closest beacon is at {beacon}")]
struct Reading
{
	sensor: Position,
	beacon: Position,
}

#[derive(
	Debug, Clone, Copy, parse_display::Display, parse_display::FromStr,
)]
#[display("x={x}, y={y}")]
struct Position
{
	x: i32,
	y: i32,
}

type Range = std::ops::RangeInclusive<i32>;

fn get_scan_intersection(
	reading: &Reading,
	scanline_y: i32,
) -> [Option<Range>; 2]
{
	let dis = manhattan_distance(reading.sensor, reading.beacon);
	let dy = scanline_y - reading.sensor.y;
	let ady = dy.abs();
	if ady <= dis
	{
		// There are two points where the "circle" with radius dis intersects
		// the line at scanline_y: x0 and x1, possibly equal.
		// They satisfy (x - s.x).abs() + (scanline_y - s.y).abs() == dis,
		// hence (x - s.x).abs() == dis - ady,
		// hence x = s.x +- (dis - ady)
		let adx = dis - ady;
		let x0 = reading.sensor.x - adx;
		let x1 = reading.sensor.x + adx;
		assert!(x0 <= x1);
		// The beacon itself is not one of the beacon-free spaces.
		if scanline_y == reading.beacon.y
		{
			let bx = reading.beacon.x;
			if x0 == bx
			{
				[Some((bx + 1)..=x1), None]
			}
			else if x1 == bx
			{
				[Some(x0..=(bx - 1)), None]
			}
			else
			{
				[Some(x0..=(bx - 1)), Some((bx + 1)..=x1)]
			}
		}
		else
		{
			[Some(x0..=x1), None]
		}
	}
	else
	{
		[None, None]
	}
}

fn manhattan_distance(from: Position, to: Position) -> i32
{
	(to.x - from.x).abs() + (to.y - from.y).abs()
}

fn count_deduplicated_ranges(ranges: Vec<Range>) -> usize
{
	let mut ranges = ranges.into_iter().peekable();
	let mut num_free_spaces = 0;
	let mut merging_range: Option<Range> = None;
	while let Some(range) = ranges.next()
	{
		let range = match merging_range.take()
		{
			Some(other) =>
			{
				let start: i32 = std::cmp::min(*range.start(), *other.start());
				let end: i32 = std::cmp::max(*range.end(), *other.end());
				start..=end
			}
			None => range,
		};
		match ranges.peek()
		{
			Some(other) if overlap(&range, other) =>
			{
				merging_range = Some(range)
			}
			Some(_) | None =>
			{
				num_free_spaces += range.count();
			}
		}
	}
	num_free_spaces
}

// From day04.
fn overlap(a: &Range, b: &Range) -> bool
{
	// Edge containment is NOT symmetric (e.g. 4-4 does not contain 1 or 9).
	contains_edge(&a, &b) || contains_edge(&b, &a)
}

// From day04.
fn contains_edge(a: &Range, b: &Range) -> bool
{
	a.contains(&b.start()) || a.contains(&b.end())
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
		assert_eq!(one(PROVIDED, 20), 26);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED, 20), 56000011);
	}
}
