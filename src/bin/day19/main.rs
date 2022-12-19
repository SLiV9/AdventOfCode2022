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
	let mut max_num_geodes = 0;
	let mut strategy = Strategy::default();
	for _ in 0..MAX_TIME
	{
		dbg!(strategy);
		let outcome = run_simulation(blueprint, strategy);
		dbg!(outcome);
		if outcome.num_geodes > max_num_geodes
		{
			max_num_geodes = outcome.num_geodes;
		}
		if outcome.num_penalties == 0
		{
			break;
		}
		strategy.max_additional_ore_robots += 1;
	}
	max_num_geodes
}

const MAX_TIME: i32 = 24;

#[derive(Debug, Default, Clone, Copy)]
struct Strategy
{
	max_additional_ore_robots: i32,
}

#[derive(Debug, Default, Clone, Copy)]
struct Outcome
{
	num_geodes: i32,
	num_penalties: i32,
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
	let mut num_penalties = 0;
	let Strategy {
		max_additional_ore_robots,
	} = strategy;
	let mut additional_ore_robots = max_additional_ore_robots;
	for time_remaining in (0..MAX_TIME).rev()
	{
		// Choose what mining robot to buy.
		let mut miner_type = None;
		if obsidian >= blueprint.geode_robot_obsidian_cost
		{
			if ore >= blueprint.geode_robot_ore_cost
			{
				miner_type = Some(ResourceType::Geode);
				ore -= blueprint.geode_robot_ore_cost;
				obsidian -= blueprint.geode_robot_obsidian_cost;
			}
			else
			{
				num_penalties += 1;
			}
		}
		if miner_type.is_none()
			&& clay >= blueprint.obsidian_robot_clay_cost
			&& time_remaining > blueprint.geode_robot_obsidian_cost + 2
		{
			if ore >= blueprint.obsidian_robot_ore_cost
			{
				miner_type = Some(ResourceType::Obsidian);
				ore -= blueprint.obsidian_robot_ore_cost;
				clay -= blueprint.obsidian_robot_clay_cost;
			}
			else
			{
				num_penalties += 1;
			}
		}
		if miner_type.is_none()
		{
			if additional_ore_robots == 0
				&& ore >= blueprint.clay_robot_ore_cost
			{
				miner_type = Some(ResourceType::Clay);
				ore -= blueprint.clay_robot_ore_cost;
			}
			else if ore >= blueprint.ore_robot_ore_cost
			{
				miner_type = Some(ResourceType::Ore);
				ore -= blueprint.ore_robot_ore_cost;
				additional_ore_robots -= 1;
			}
		}
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
	dbg!(ore);
	dbg!(clay);
	dbg!(obsidian);
	Outcome {
		num_geodes: geodes,
		num_penalties,
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
		assert_eq!(one(PROVIDED), 33);
	}
}
