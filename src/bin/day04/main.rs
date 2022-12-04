/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> usize
{
	input
		.lines()
		.map(|line| line.parse().unwrap())
		.filter(|pairing| fully_contained_either_way(pairing))
		.count()
}

#[derive(parse_display::Display, parse_display::FromStr)]
#[display("{a},{b}")]
struct Pairing
{
	a: Range,
	b: Range,
}

#[derive(parse_display::Display, parse_display::FromStr)]
#[display("{start}-{end}")]
struct Range
{
	start: i32,
	end: i32,
}

fn fully_contained_either_way(pairing: &Pairing) -> bool
{
	fully_contains(&pairing.a, &pairing.b)
		|| fully_contains(&pairing.b, &pairing.a)
}

fn fully_contains(a: &Range, b: &Range) -> bool
{
	(a.start..=a.end).contains(&b.start) && (a.start..=a.end).contains(&b.end)
}

fn overlap_either_way(pairing: &Pairing) -> bool
{
	// Edge containment is NOT symmetric (e.g. 4-4 does not contain 1 or 9).
	contains_edge(&pairing.a, &pairing.b)
		|| contains_edge(&pairing.b, &pairing.a)
}

fn contains_edge(a: &Range, b: &Range) -> bool
{
	(a.start..=a.end).contains(&b.start) || (a.start..=a.end).contains(&b.end)
}

fn two(input: &str) -> usize
{
	input
		.lines()
		.map(|line| line.parse().unwrap())
		.filter(|pairing| overlap_either_way(pairing))
		.count()
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
		assert_eq!(one(PROVIDED), 2);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 4);
	}
}
