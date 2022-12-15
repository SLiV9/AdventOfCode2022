/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT, 2000000));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str, scanline_y: i32) -> usize
{
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

fn two(_input: &str) -> i32
{
	0
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
		assert_eq!(one(PROVIDED, 10), 26);
	}
}
