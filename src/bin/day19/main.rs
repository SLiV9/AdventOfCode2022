/**/

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

fn two(_input: &str) -> i32
{
	0
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
	dbg!(blueprint.blueprint_id) * dbg!(optimize_num_geodes(blueprint))
}

fn optimize_num_geodes(blueprint: Blueprint) -> i32
{
	let strategies = (1..MAX_TIME)
		.flat_map(|o| (1..(MAX_TIME - o)).map(move |c| (o, c)))
		.flat_map(|(o, c)| (0..(MAX_TIME - o - c)).map(move |x| (o, c, x)))
		.map(|(o, c, x)| Strategy {
			max_additional_ore_robots: x,
			max_clay_robots: c,
			max_obsidian_robots: o,
		});
	let mut max_num_geodes = 0;
	for strategy in strategies
	{
		dbg!(strategy);
		let outcome = run_simulation(blueprint, strategy);
		dbg!(outcome);
		if outcome.num_geodes > max_num_geodes
		{
			max_num_geodes = outcome.num_geodes;
		}
	}
	max_num_geodes
}

const MAX_TIME: i32 = 24;

#[derive(Debug, Default, Clone, Copy)]
struct Strategy
{
	max_additional_ore_robots: i32,
	max_clay_robots: i32,
	max_obsidian_robots: i32,
}

#[derive(Debug, Default, Clone, Copy)]
struct Outcome
{
	num_geodes: i32,
}

#[derive(Debug, Clone, Copy)]
enum ResourceType
{
	Ore,
	Clay,
	Obsidian,
	Geode,
}

fn run_simulation(blueprint: Blueprint, strategy: Strategy) -> Outcome
{
	let mut num_ore_robots = 1;
	let mut num_clay_robots = 0;
	let mut num_obsidian_robots = 0;
	let mut num_geode_robots = 0;
	let mut ore = 0;
	let mut clay = 0;
	let mut obsidian = 0;
	let mut geodes = 0;
	let Strategy {
		max_additional_ore_robots,
		max_clay_robots,
		max_obsidian_robots,
	} = strategy;
	let mut additional_ore_robots = max_additional_ore_robots;
	let mut additional_clay_robots = max_clay_robots;
	let mut additional_obsidian_robots = max_obsidian_robots;
	for _time_remaining in (0..MAX_TIME).rev()
	{
		// Choose what mining robot to buy.
		let miner_type = if ore >= blueprint.geode_robot_ore_cost
			&& obsidian >= blueprint.geode_robot_obsidian_cost
		{
			ore -= blueprint.geode_robot_ore_cost;
			obsidian -= blueprint.geode_robot_obsidian_cost;
			Some(ResourceType::Geode)
		}
		else if additional_obsidian_robots > 0
			&& ore >= blueprint.obsidian_robot_ore_cost
			&& clay >= blueprint.obsidian_robot_clay_cost
		{
			additional_obsidian_robots -= 1;
			ore -= blueprint.obsidian_robot_ore_cost;
			clay -= blueprint.obsidian_robot_clay_cost;
			Some(ResourceType::Obsidian)
		}
		else if additional_clay_robots > 0
			&& ore >= blueprint.clay_robot_ore_cost
		{
			additional_clay_robots -= 1;
			ore -= blueprint.clay_robot_ore_cost;
			Some(ResourceType::Clay)
		}
		else if additional_ore_robots > 0
			&& ore >= blueprint.ore_robot_ore_cost
		{
			additional_ore_robots -= 1;
			ore -= blueprint.ore_robot_ore_cost;
			Some(ResourceType::Ore)
		}
		else
		{
			None
		};
		// Gain resources.
		ore += num_ore_robots;
		clay += num_clay_robots;
		obsidian += num_obsidian_robots;
		geodes += num_geode_robots;
		// Finish construction.
		if let Some(miner_type) = miner_type
		{
			match miner_type
			{
				ResourceType::Ore => num_ore_robots += 1,
				ResourceType::Clay => num_clay_robots += 1,
				ResourceType::Obsidian => num_obsidian_robots += 1,
				ResourceType::Geode => num_geode_robots += 1,
			}
		}
	}
	//dbg!(ore);
	//dbg!(clay);
	//dbg!(obsidian);
	Outcome { num_geodes: geodes }
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
		let strategy = Strategy {
			max_additional_ore_robots: 0,
			max_clay_robots: 4,
			max_obsidian_robots: 2,
		};
		let outcome = run_simulation(blueprint, strategy);
		assert_eq!(outcome.num_geodes, 9);
	}
}
