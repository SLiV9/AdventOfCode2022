/**/

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> usize
{
	let mut game: KeepAway = input.parse().unwrap();
	for _i in 0..20
	{
		game.play_round();
	}
	game.level_of_monkey_business()
}

fn two(_input: &str) -> i32
{
	0
}

#[derive(Debug)]
struct KeepAway
{
	monkeys: Vec<Monkey>,
}

const MONKEY_REGEX: &str = "Monkey [0-9]+:
  Starting items: (?P<items>[0-9]+(, [0-9]+)*)
(?P<spec>  .*(\n  .*)+)";

impl std::str::FromStr for KeepAway
{
	type Err = std::io::Error;

	fn from_str(input: &str) -> Result<Self, Self::Err>
	{
		let monkey_regex = regex::Regex::new(MONKEY_REGEX).unwrap();
		let items_regex = regex::Regex::new("[0-9]+").unwrap();
		let monkeys = input
			.split("\n\n")
			.map(|segment| {
				let captures = monkey_regex.captures(segment).unwrap();
				let items_str = captures.name("items").unwrap();
				let items: Vec<i32> = items_regex
					.captures_iter(items_str.as_str())
					.map(|x| x.get(0).unwrap().as_str().parse().unwrap())
					.collect();
				let spec = captures.name("spec").unwrap();
				let spec: Specification = spec.as_str().parse().unwrap();
				Monkey {
					items,
					spec,
					number_of_inspections: 0,
				}
			})
			.collect();
		Ok(Self { monkeys })
	}
}

impl KeepAway
{
	fn play_round(&mut self)
	{
		unimplemented!()
	}

	fn level_of_monkey_business(&self) -> usize
	{
		let mut a = 1;
		let mut b = 1;
		for monkey in &self.monkeys
		{
			let n = monkey.number_of_inspections;
			if n > a
			{
				b = a;
				a = n;
			}
			else if n > b
			{
				b = n;
			}
		}
		a * b
	}
}

#[derive(Debug)]
struct Monkey
{
	items: Vec<i32>,
	spec: Specification,

	number_of_inspections: usize,
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[display(
	"  Operation: new = {operation}
  Test: divisible by {test_divisor}
    If true: throw to monkey {true_friend}
    If false: throw to monkey {false_friend}"
)]
struct Specification
{
	operation: Operation,
	test_divisor: i32,
	true_friend: usize,
	false_friend: usize,
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[display("{left} {op} {right}")]
struct Operation
{
	left: Operand,
	op: Operator,
	right: Operand,
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
enum Operand
{
	#[display("old")]
	Old,
	#[display("{0}")]
	Value(i32),
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
enum Operator
{
	#[display("+")]
	Plus,
	#[display("*")]
	Times,
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");

	#[test]
	fn parse_specification()
	{
		let spec = Specification {
			operation: Operation {
				left: Operand::Old,
				op: Operator::Times,
				right: Operand::Value(19),
			},
			test_divisor: 23,
			true_friend: 2,
			false_friend: 3,
		};
		let output = format!("{}", spec);
		let spec2: Specification = output.parse().unwrap();
		let output2 = format!("{}", spec2);
		assert_eq!(&output, &output2);
	}

	#[test]
	fn parse_operation()
	{
		let text = "old * 19";
		let operation = Operation {
			left: Operand::Old,
			op: Operator::Times,
			right: Operand::Value(19),
		};
		let output = format!("{}", operation);
		assert_eq!(&output, text);
		let operation2: Operation = output.parse().unwrap();
		let output2 = format!("{}", operation2);
		assert_eq!(&output, &output2);
	}

	#[test]
	fn parse_operator()
	{
		let text = "*";
		let operator = Operator::Times;
		let output = format!("{}", operator);
		assert_eq!(&output, text);
		let operator2: Operator = output.parse().unwrap();
		let output2 = format!("{}", operator2);
		assert_eq!(&output, &output2);
	}

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 10605);
	}
}
