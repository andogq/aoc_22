use crate::day::Day;
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Clone, Debug)]
pub struct MaterialCollection {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}
impl MaterialCollection {
    pub fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    pub fn mine(&mut self, material: Material, amount: usize) {
        match material {
            Material::Ore => self.ore += amount,
            Material::Clay => self.clay += amount,
            Material::Obsidian => self.obsidian += amount,
            Material::Geode => self.geode += amount,
        }
    }

    pub fn get(&self, material: &Material) -> usize {
        match material {
            Material::Ore => self.ore,
            Material::Clay => self.clay,
            Material::Obsidian => self.obsidian,
            Material::Geode => self.geode,
        }
    }

    pub fn max(&mut self, other: &Self) {
        self.ore = self.ore.max(other.ore);
        self.clay = self.clay.max(other.clay);
        self.obsidian = self.obsidian.max(other.obsidian);
        self.geode = self.geode.max(other.geode);
    }
}
impl Add<&MaterialCollection> for MaterialCollection {
    type Output = Self;

    fn add(self, rhs: &MaterialCollection) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}
impl AddAssign for MaterialCollection {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}
impl Sub<&MaterialCollection> for MaterialCollection {
    type Output = Self;

    fn sub(self, rhs: &MaterialCollection) -> Self::Output {
        Self {
            ore: self.ore.saturating_sub(rhs.ore),
            clay: self.clay.saturating_sub(rhs.clay),
            obsidian: self.obsidian.saturating_sub(rhs.obsidian),
            geode: self.geode.saturating_sub(rhs.geode),
        }
    }
}
impl SubAssign for MaterialCollection {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore = self.ore.saturating_sub(rhs.ore);
        self.clay = self.clay.saturating_sub(rhs.clay);
        self.obsidian = self.obsidian.saturating_sub(rhs.obsidian);
        self.geode = self.geode.saturating_sub(rhs.geode);
    }
}
impl FromIterator<(Material, usize)> for MaterialCollection {
    fn from_iter<T: IntoIterator<Item = (Material, usize)>>(iter: T) -> Self {
        let mut collection = MaterialCollection::new();

        for (material, amount) in iter.into_iter() {
            collection.mine(material, amount);
        }

        collection
    }
}

#[derive(Clone, Debug)]
pub struct Blueprint {
    id: usize,
    robots: Vec<RobotBlueprint>,
}

impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        let (lhs, rhs) = value.split_once(':').unwrap();

        let id = lhs.split_once(' ').unwrap().1.parse().unwrap();
        let robots = rhs
            .split('.')
            .take_while(|s| !s.is_empty())
            .map(RobotBlueprint::from)
            .collect();

        Self { id, robots }
    }
}

#[derive(Clone, Debug)]
pub struct RobotBlueprint {
    robot: Material,
    requirements: MaterialCollection,
}

impl From<&str> for RobotBlueprint {
    fn from(value: &str) -> Self {
        let (lhs, rhs) = value.split_once(" robot costs ").unwrap();

        let robot = lhs.trim().split_once(' ').unwrap().1.into();

        let requirements = rhs
            .trim_matches('.')
            .split(" and ")
            .map(|req| {
                let (amount, material) = req.split_once(' ').unwrap();

                let amount: usize = amount.parse().unwrap();
                let material: Material = material.into();

                (material, amount)
            })
            .collect();

        Self {
            robot,
            requirements,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
impl Material {
    #[inline]
    pub fn options() -> impl Iterator<Item = Self> {
        use Material::*;

        [Ore, Clay, Obsidian, Geode].into_iter()
    }
}
impl From<&str> for Material {
    fn from(value: &str) -> Self {
        use Material::*;

        match value {
            "ore" => Ore,
            "clay" => Clay,
            "obsidian" => Obsidian,
            "geode" => Geode,
            _ => unreachable!(),
        }
    }
}

fn solve(blueprints: &[Blueprint], time: usize) -> HashMap<usize, usize> {
    #[derive(Debug)]
    struct State<'a> {
        robots: MaterialCollection,
        materials: MaterialCollection,
        time_remaining: usize,
        blueprint: &'a Blueprint,
    }

    let mut frontier = VecDeque::new();
    for blueprint in blueprints {
        frontier.push_back(State {
            robots: MaterialCollection::from_iter([(Material::Ore, 1)]),
            materials: MaterialCollection::new(),
            time_remaining: time,
            blueprint,
        });
    }

    let mut largest: HashMap<usize, usize> = HashMap::new();

    while let Some(state) = frontier.pop_front() {
        // Determine the maximum requirements for each material type
        let max_requirements = state.blueprint.robots.iter().fold(
            MaterialCollection::new(),
            |mut max_requirements, blueprint| {
                max_requirements.max(&blueprint.requirements);
                max_requirements
            },
        );

        // Calculate time for each possible type of robot to be generated
        let next_states = state.blueprint.robots.iter().filter_map(|robot_blueprint| {
            // Check if there's a current need for the robot
            if robot_blueprint.robot != Material::Geode
                && state.robots.get(&robot_blueprint.robot)
                    >= max_requirements.get(&robot_blueprint.robot)
            {
                return None;
            }

            let material_times = Material::options()
                .filter_map(|material| {
                    let required_material = robot_blueprint.requirements.get(&material);
                    let available_material = state.materials.get(&material);

                    if required_material > available_material {
                        // Still have to produce material
                        let remaining_material = required_material - available_material;

                        let material_production = state.robots.get(&material);
                        if material_production > 0 {
                            return Some(
                                (remaining_material / material_production)
                                    + if remaining_material % material_production == 0 {
                                        0
                                    } else {
                                        1
                                    },
                            );
                        }

                        None
                    } else {
                        // Robot can be immediately built
                        Some(0)
                    }
                })
                .collect::<Vec<_>>();

            // Check if it's possible to create all materials
            if material_times.len() == 4 {
                // Add one step for making robot
                let time_to_create = material_times.into_iter().max().unwrap() + 1;

                if time_to_create <= state.time_remaining {
                    let new_state = State {
                        time_remaining: state.time_remaining - time_to_create,
                        materials: {
                            let mut materials = state.materials.clone();

                            // Mine materials
                            Material::options().for_each(|material| {
                                materials
                                    .mine(material, state.robots.get(&material) * time_to_create)
                            });

                            // Spend on robot creation
                            materials -= robot_blueprint.requirements.clone();

                            materials
                        },
                        robots: {
                            let mut robots = state.robots.clone();
                            robots.mine(robot_blueprint.robot, 1);
                            robots
                        },
                        blueprint: state.blueprint,
                    };

                    // Make sure it's possible to beat the current best
                    let max_geodes = new_state.materials.get(&Material::Geode) // Current geodes
                            // Geodes produced
                        + (new_state.robots.get(&Material::Geode) * new_state.time_remaining)
                            // Max possible if geode robot greated every turn
                        + (new_state.time_remaining * (new_state.time_remaining - 1) / 2);

                    if largest
                        .get(&new_state.blueprint.id)
                        .cloned()
                        .unwrap_or_default()
                        >= max_geodes
                    {
                        return None;
                    }

                    if new_state.time_remaining == 0 {
                        let largest = largest.entry(new_state.blueprint.id).or_default();
                        *largest = *largest.max(&mut new_state.materials.geode.to_owned());

                        None
                    } else {
                        Some(new_state)
                    }
                } else {
                    None
                }
            } else {
                None
            }
        });

        frontier.extend(next_states);
    }

    largest
}

pub struct Day19;
impl Day for Day19 {
    type Input = Vec<Blueprint>;
    type Output = usize;

    fn part_1(input: Self::Input) -> Self::Output {
        let largest = solve(&input, 24);

        dbg!(&largest);

        largest.into_iter().map(|(id, amount)| id * amount).sum()
    }

    fn part_2(input: Self::Input) -> Self::Output {
        let largest = solve(&input.into_iter().take(3).collect::<Vec<_>>(), 32);

        dbg!(&largest);

        largest.values().product()
    }

    fn parse(raw: &str) -> Self::Input {
        raw.lines().map(Blueprint::from).collect()
    }
}

#[test]
fn test() {
    let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    assert_eq!(Day19::run(input), (33, 3472));
}
