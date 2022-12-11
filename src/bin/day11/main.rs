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
	dbg!(&game);
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

impl std::str::FromStr for KeepAway
{
	type Err = std::io::Error;

	fn from_str(input: &str) -> Result<Self, Self::Err>
	{
		let monkeys = input
			.split("\n\n")
			.map(|x| x.trim().parse().unwrap())
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

impl std::str::FromStr for Monkey
{
	type Err = std::io::Error;

	fn from_str(input: &str) -> Result<Self, Self::Err>
	{
		dbg!(input);
		let template: MonkeyTemplate = input.parse().unwrap();
		let MonkeyTemplate {
			number: _,
			items_str,
			spec,
		} = template;
		let items = items_str
			.0
			.split(", ")
			.map(|x| x.parse().unwrap())
			.collect();
		Ok(Monkey {
			items: items,
			spec,
			number_of_inspections: 0,
		})
	}
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[display("Monkey {number}:\n  Starting items: {items_str}\n{spec}")]
struct MonkeyTemplate
{
	number: usize,
	items_str: ItemsStr,
	spec: Specification,
}

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[from_str(regex = "(?P<0>[0-9]+(, [0-9]+)*)")]
struct ItemsStr(String);

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
	fn parse_template()
	{
		let template = MonkeyTemplate {
			number: 0,
			items_str: ItemsStr(String::from("79, 98")),
			spec: Specification {
				operation: Operation {
					left: Operand::Old,
					op: Operator::Times,
					right: Operand::Value(19),
				},
				test_divisor: 23,
				true_friend: 2,
				false_friend: 3,
			},
		};
		let output = format!("{}\n", template);
		assert_eq!(&output, include_str!("template_sample.txt"));
		let template2: MonkeyTemplate = output.trim().parse().unwrap();
		let output2 = format!("{}\n", template2);
		assert_eq!(&output, &output2);
	}

	#[test]
	fn parse_items()
	{
		let text = "79, 98";
		let items = ItemsStr(String::from(text));
		let output = format!("{}", items);
		assert_eq!(&output, text);
		let items2: ItemsStr = output.parse().unwrap();
		let output2 = format!("{}", items2);
		assert_eq!(&output, &output2);
	}

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
		dbg!(&output);
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

	#[derive(Debug, parse_display::Display, parse_display::FromStr)]
	#[display("Monkey 0:\n  Starting items: 79, 98\n{spec}")]
	struct DummyTemplate
	{
		spec: Specification,
	}
	#[test]
	fn parse_dummy()
	{
		let template = DummyTemplate {
			spec: Specification {
				operation: Operation {
					left: Operand::Old,
					op: Operator::Times,
					right: Operand::Value(19),
				},
				test_divisor: 23,
				true_friend: 2,
				false_friend: 3,
			},
		};
		let output = format!("{}\n", template);
		assert_eq!(&output, include_str!("template_sample.txt"));
		let template2: DummyTemplate = output.trim().parse().unwrap();
		let output2 = format!("{}\n", template2);
		assert_eq!(&output, &output2);
	}

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 10605);
	}
}
