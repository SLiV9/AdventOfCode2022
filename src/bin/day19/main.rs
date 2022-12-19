/**/

use smallvec::SmallVec;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	println!("Part One: {}", one(INPUT));
	println!("Part Two: {}", two(INPUT));
}

fn one(input: &str) -> i32
{
	let blueprints = input.lines().map(|line| line.parse().unwrap());
	blueprints.map(determine_quality_level).sum()
}

fn two(input: &str) -> i32
{
	let blueprints = input.lines().take(3).map(|line| line.parse().unwrap());
	blueprints
		.map(|blueprint| optimize_num_geodes(blueprint, MAX_TIME_PART_TWO))
		.product()
}

#[derive(Debug, Default, Clone, Copy)]
#[derive(parse_display::Display, parse_display::FromStr)]
#[display(
	"Blueprint {blueprint_id}: Each ore robot costs {ore_robot_ore_cost} ore. \
	 Each clay robot costs {clay_robot_ore_cost} ore. Each obsidian robot \
	 costs {obsidian_robot_ore_cost} ore and {obsidian_robot_clay_cost} clay. \
	 Each geode robot costs {geode_robot_ore_cost} ore and \
	 {geode_robot_obsidian_cost} obsidian."
)]
struct Blueprint
{
	blueprint_id: i32,
	ore_robot_ore_cost: i32,
	clay_robot_ore_cost: i32,
	obsidian_robot_ore_cost: i32,
	obsidian_robot_clay_cost: i32,
	geode_robot_ore_cost: i32,
	geode_robot_obsidian_cost: i32,
}

fn determine_quality_level(blueprint: Blueprint) -> i32
{
	dbg!(blueprint.blueprint_id)
		* dbg!(optimize_num_geodes(blueprint, MAX_TIME_PART_ONE))
}

fn optimize_num_geodes(blueprint: Blueprint, time_allowed: i32) -> i32
{
	let mut starting_state = State::default();
	starting_state.world.num_ore_robots = 1;
	starting_state.time_remaining = time_allowed;
	let mut max_num_geodes = 0;
	let mut stack = Vec::new();
	stack.push(starting_state);
	while let Some(state) = stack.pop()
	{
		//dbg!(&state);
		let num_geodes =
			run_simulation(blueprint, state, &mut stack, Some(max_num_geodes));
		if num_geodes > max_num_geodes
		{
			max_num_geodes = num_geodes;
		}
	}
	max_num_geodes
}

const MAX_TIME_PART_ONE: i32 = 24;
const MAX_TIME_PART_TWO: i32 = 32;
const MAX_STRATEGY_LEN: usize = MAX_TIME_PART_TWO as usize;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Strategy
{
	choices: SmallVec<[ResourceType; MAX_STRATEGY_LEN]>,
}

impl std::hash::Hash for Strategy
{
	fn hash<H: std::hash::Hasher>(&self, state: &mut H)
	{
		self.choices[..].hash(state)
	}
}

impl Strategy
{
	#[cfg(test)]
	fn parse(input: &str) -> Strategy
	{
		let choices = input
			.chars()
			.map(|x| x.to_string().parse().unwrap())
			.collect();
		Strategy { choices }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(parse_display::Display, parse_display::FromStr)]
#[repr(u8)]
enum ResourceType
{
	#[display("x")]
	Ore,
	#[display("c")]
	Clay,
	#[display("o")]
	Obsidian,
	#[display("g")]
	Geode,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct World
{
	num_ore_robots: i32,
	num_clay_robots: i32,
	num_obsidian_robots: i32,
	num_geode_robots: i32,
	ore: i32,
	clay: i32,
	obsidian: i32,
	geodes: i32,
}

impl World
{
	fn can_afford(&self, robot_type: ResourceType, blueprint: Blueprint)
		-> bool
	{
		match robot_type
		{
			ResourceType::Geode =>
			{
				self.ore >= blueprint.geode_robot_ore_cost
					&& self.obsidian >= blueprint.geode_robot_obsidian_cost
			}
			ResourceType::Obsidian =>
			{
				self.ore >= blueprint.obsidian_robot_ore_cost
					&& self.clay >= blueprint.obsidian_robot_clay_cost
			}
			ResourceType::Clay => self.ore >= blueprint.clay_robot_ore_cost,
			ResourceType::Ore => self.ore >= blueprint.ore_robot_ore_cost,
		}
	}

	fn pay(&mut self, robot_type: ResourceType, blueprint: Blueprint)
	{
		match robot_type
		{
			ResourceType::Geode =>
			{
				self.ore -= blueprint.geode_robot_ore_cost;
				self.obsidian -= blueprint.geode_robot_obsidian_cost;
			}
			ResourceType::Obsidian =>
			{
				self.ore -= blueprint.obsidian_robot_ore_cost;
				self.clay -= blueprint.obsidian_robot_clay_cost;
			}
			ResourceType::Clay =>
			{
				self.ore -= blueprint.clay_robot_ore_cost;
			}
			ResourceType::Ore =>
			{
				self.ore -= blueprint.ore_robot_ore_cost;
			}
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct State
{
	world: World,
	strategy: Strategy,
	strategy_offset: usize,
	time_remaining: i32,
}

fn run_simulation(
	blueprint: Blueprint,
	mut state: State,
	stack: &mut Vec<State>,
	max_num_geodes: Option<i32>,
) -> i32
{
	while state.time_remaining > 0
	{
		if let Some(max_num_geodes) = max_num_geodes
		{
			let t = state.time_remaining;
			let loose_upper_limit = state.world.geodes
				+ t * state.world.num_geode_robots
				+ (t - 1) * t / 2;
			if loose_upper_limit < max_num_geodes
			{
				break;
			}
		}

		step(blueprint, &mut state, stack);
	}
	state.world.geodes
}

fn step(blueprint: Blueprint, state: &mut State, stack: &mut Vec<State>)
{
	let world = &mut state.world;
	let strategy = &mut state.strategy;

	// Choose what mining robot to buy next.
	let dictated_type = strategy.choices.get(state.strategy_offset).cloned();
	let chosen_type = if let Some(dictated_type) = dictated_type
	{
		dictated_type
	}
	else
	{
		let chosen_type = if world.num_obsidian_robots > 0
		{
			ResourceType::Geode
		}
		else if world.num_clay_robots > 0
		{
			ResourceType::Obsidian
		}
		else
		{
			ResourceType::Clay
		};
		let alternative_choices = [
			ResourceType::Ore,
			ResourceType::Clay,
			ResourceType::Obsidian,
		]
		.into_iter()
		.filter(|x| *x < chosen_type)
		.filter(|x| match *x
		{
			ResourceType::Clay =>
			{
				let t = state.time_remaining - 4;
				world.clay + t * world.num_clay_robots
					< t * blueprint.obsidian_robot_clay_cost
			}
			ResourceType::Obsidian =>
			{
				let t = state.time_remaining - 2;
				world.obsidian + t * world.num_obsidian_robots
					< t * blueprint.geode_robot_obsidian_cost
			}
			_ => true,
		});
		for other in alternative_choices
		{
			let mut alt_strategy = strategy.clone();
			alt_strategy.choices.push(other);
			let alternative = State {
				world: world.clone(),
				strategy: alt_strategy,
				strategy_offset: state.strategy_offset,
				time_remaining: state.time_remaining,
			};
			stack.push(alternative);
		}
		strategy.choices.push(chosen_type);
		chosen_type
	};
	// Pay for construction if we can afford the chosen mining robot, or wait.
	let miner_type = if world.can_afford(chosen_type, blueprint)
	{
		world.pay(chosen_type, blueprint);
		Some(chosen_type)
	}
	else
	{
		None
	};
	// Gain resources.
	world.ore += world.num_ore_robots;
	world.clay += world.num_clay_robots;
	world.obsidian += world.num_obsidian_robots;
	world.geodes += world.num_geode_robots;
	// Finish construction.
	if let Some(miner_type) = miner_type
	{
		match miner_type
		{
			ResourceType::Ore => world.num_ore_robots += 1,
			ResourceType::Clay => world.num_clay_robots += 1,
			ResourceType::Obsidian => world.num_obsidian_robots += 1,
			ResourceType::Geode => world.num_geode_robots += 1,
		}
		state.strategy_offset += 1;
	}
	// Tick.
	state.time_remaining -= 1;
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");
	const PROVIDED1: &str = include_str!("provided1.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 33);
	}

	#[test]
	fn one_provided1()
	{
		assert_eq!(one(PROVIDED1), 9);
	}

	#[test]
	fn one_provided1_provided_strategy()
	{
		let blueprint = PROVIDED1.lines().next().unwrap().parse().unwrap();
		let strategy = Strategy::parse("cccocoggg");
		let mut state = State::default();
		state.world.num_ore_robots = 1;
		state.time_remaining = MAX_TIME_PART_ONE;
		state.strategy = strategy;
		let mut stack = Vec::new();
		let num_geodes = run_simulation(blueprint, state, &mut stack, None);
		assert_eq!(num_geodes, 9);
		assert!(stack.is_empty());
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 56 * 62);
	}

	#[test]
	fn two_provided1()
	{
		assert_eq!(two(PROVIDED1), 56);
	}
}
