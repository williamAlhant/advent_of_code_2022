use crate::days::internal_common::*;

pub fn day_19_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let blueprints = parse::parse_and_collect(&input)?;
    
    let mut sum = 0;

    for (i, bp) in blueprints.iter().enumerate() {
        let bp_id = i + 1;
        println!("bp id {} {:?}", bp_id, bp);
        let mut solv = BBSolv::new(bp.clone(), 24);
        solv.solve();
        if solv.sol_found {
            println!("bp solution {}", solv.lower_bound);
            sum += bp_id * solv.lower_bound as usize;
        }
        else {
            println!("no solution");
        }
    }

    println!("Sum {}", sum);

    Ok(())
}

pub fn day_19_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let blueprints = parse::parse_and_collect(&input)?;
    
    let mut prod = 1;

    for bp in blueprints.iter().take(3) {
        println!("bp {:?}", bp);
        let mut solv = BBSolv::new(bp.clone(), 32);
        solv.solve();
        if solv.sol_found {
            println!("bp solution {}", solv.lower_bound);
            prod *= solv.lower_bound as usize;
        }
        else {
            println!("no solution");
        }
    }

    println!("Prod {}", prod);

    Ok(())
}

#[derive(Clone, Debug)]
struct Blueprint {
    ore_cost_in_ore: u8,
    clay_cost_in_ore: u8,
    obsi_cost_in_ore: u8,
    geo_cost_in_ore: u8,
    obsi_cost_in_clay: u8,
    geo_cost_in_obsi: u8,
}

#[derive(Clone, Copy, Debug)]
struct ResourceList {
    ore: u8,
    clay: u8,
    obsi: u8,
    geo: u8,
}

#[derive(Clone, Debug)]
struct FullResourceList {
    mineral: ResourceList,
    robot: ResourceList
}

type NodeId = usize;

#[derive(Clone, Debug)]
struct BBNode {
    parent: NodeId,
    depth: u8,
    resource_list: FullResourceList
}

struct BBSolv {
    nodes: Vec<BBNode>,
    todo: Vec<NodeId>,
    lower_bound: i32,
    best_node: NodeId,
    blueprint: Blueprint,
    sol_found: bool,
    final_depth: u8
}

impl BBSolv {
    const INIT_LOWER_BOUND: i32 = 1;

    fn new(blueprint: Blueprint, final_depth: u8) -> Self
    {
        let mut ret = BBSolv {
            nodes: Vec::with_capacity(10_000),
            todo: Vec::new(),
            lower_bound: BBSolv::INIT_LOWER_BOUND,
            best_node: 0,
            blueprint,
            sol_found: false,
            final_depth
        };

        let root = BBNode {
            parent: 0,
            depth: 0,
            resource_list: FullResourceList {
                mineral: ResourceList { ore: 0, clay: 0, obsi: 0, geo: 0 },
                robot: ResourceList { ore: 1, clay: 0, obsi: 0, geo: 0 }
            }
        };

        ret.nodes.push(root);
        ret.todo.push(0);

        ret
    }

    fn step(&mut self)
    {
        let node_id = self.todo.pop().unwrap();
        let node = self.nodes[node_id].clone();
        let list_next = self.possible_resource_list_next(&node.resource_list);
        for next in list_next {
            // BBSolv::print_decision(&node.resource_list, &next);
            let node = BBNode {
                parent: node_id,
                depth: node.depth + 1,
                resource_list: next
            };
            let ub = self.compute_some_upper_bound(&node);
            if (ub as i32) < self.lower_bound {
                // println!("SKIP");
                continue;
            }
            if node.depth == self.final_depth {
                let score = node.resource_list.mineral.geo as i32;
                // dbg!(&node, score);
                if score >= self.lower_bound {
                    self.lower_bound = score;
                    self.nodes.push(node);
                    self.best_node = self.nodes.len() - 1;
                    self.sol_found = true;
                }
                continue;
            }
            self.nodes.push(node);
            self.todo.push(self.nodes.len() - 1);
        }
    }

    fn solve(&mut self)
    {
        let print_every_n_steps = 10_000;
        let mut steps_since_last_print = 0;
        while !self.todo.is_empty() {
            self.step();
            steps_since_last_print += 1;
            if steps_since_last_print == print_every_n_steps {
                println!("Processed {} nodes", self.nodes.len());
                steps_since_last_print = 0;
            }
        }
    }

    fn compute_some_upper_bound(&self, node: &BBNode) -> u8
    {
        let mut resource_list = node.resource_list.clone();
        for _i in node.depth..self.final_depth {

            let add_r_ore = resource_list.mineral.ore / self.blueprint.ore_cost_in_ore.min(self.blueprint.clay_cost_in_ore);
            let add_r_clay = resource_list.mineral.ore / self.blueprint.clay_cost_in_ore;
            let add_r_obsi = resource_list.mineral.clay / self.blueprint.obsi_cost_in_clay;
            let add_r_geo = resource_list.mineral.obsi / self.blueprint.geo_cost_in_obsi;

            // dbg!(_i, &resource_list);
            resource_list = FullResourceList {
                mineral: ResourceList {
                    ore: resource_list.mineral.ore + resource_list.robot.ore - add_r_clay * self.blueprint.clay_cost_in_ore,
                    clay: resource_list.mineral.clay + resource_list.robot.clay - add_r_obsi * self.blueprint.obsi_cost_in_clay,
                    obsi: resource_list.mineral.obsi + resource_list.robot.obsi - add_r_geo * self.blueprint.geo_cost_in_obsi,
                    geo: resource_list.mineral.geo + resource_list.robot.geo,
                },
                robot: ResourceList {
                    ore: resource_list.robot.ore + add_r_ore,
                    clay: resource_list.robot.clay + add_r_clay,
                    obsi: resource_list.robot.obsi + add_r_obsi,
                    geo: resource_list.robot.geo + add_r_geo,
                }
            };

            resource_list.robot.ore = resource_list.robot.ore.min(100);
            resource_list.robot.clay = resource_list.robot.clay.min(100);
        }

        resource_list.mineral.geo
    }

    fn print_decision(before: &FullResourceList, after: &FullResourceList)
    {
        let add_r_ore = after.robot.ore - before.robot.ore;
        let add_r_clay = after.robot.clay - before.robot.clay;
        let add_r_obsi = after.robot.obsi - before.robot.obsi;
        let add_r_geo = after.robot.geo - before.robot.geo;
        println!("Decision to add {} ore {} clay {} obsi {} geo", add_r_ore, add_r_clay, add_r_obsi, add_r_geo);
    }

    #[allow(dead_code)]
    fn print_best(&self)
    {
        let mut ancestry = Vec::new();
        let mut node = &self.nodes[self.best_node];
        while node.depth != 0 {
            ancestry.push(node);
            node = &self.nodes[node.parent];
        }
        for i in (0..ancestry.len()).rev() {
            let node = ancestry[i];
            println!("Depth {}", node.depth);
            BBSolv::print_decision(&self.nodes[node.parent].resource_list, &node.resource_list);
            // dbg!(&node.resource_list);
        }
    }

    fn possible_resource_list_next(&self, current: &FullResourceList) -> Vec<FullResourceList>
    {
        let mut ret = Vec::new();

        let current_mineral = current.mineral.clone();
        let max_add_r_geo = (current_mineral.ore / self.blueprint.geo_cost_in_ore).min(
            current_mineral.obsi / self.blueprint.geo_cost_in_obsi
        );

        for add_r_geo in 0..=max_add_r_geo {
            let current_mineral = ResourceList {
                ore: current_mineral.ore - add_r_geo * self.blueprint.geo_cost_in_ore,
                clay: current_mineral.clay,
                obsi: current_mineral.obsi - add_r_geo * self.blueprint.geo_cost_in_obsi,
                geo: current_mineral.geo,
            };
            let max_add_r_obsi = (current_mineral.ore / self.blueprint.obsi_cost_in_ore).min(
                current_mineral.clay / self.blueprint.obsi_cost_in_clay
            );

            for add_r_obsi in 0..=max_add_r_obsi {
                let current_mineral = ResourceList {
                    ore: current_mineral.ore - add_r_obsi * self.blueprint.obsi_cost_in_ore,
                    clay: current_mineral.clay - add_r_obsi * self.blueprint.obsi_cost_in_clay,
                    obsi: current_mineral.obsi,
                    geo: current_mineral.geo,
                };
                let max_add_r_clay = current_mineral.ore / self.blueprint.clay_cost_in_ore;

                for add_r_clay in 0..=max_add_r_clay {
                    let current_mineral = ResourceList {
                        ore: current_mineral.ore - add_r_clay * self.blueprint.clay_cost_in_ore,
                        clay: current_mineral.clay,
                        obsi: current_mineral.obsi,
                        geo: current_mineral.geo,
                    };
                    let max_add_r_ore = current_mineral.ore / self.blueprint.ore_cost_in_ore;

                    for add_r_ore in 0..=max_add_r_ore {
                        let current_mineral = ResourceList {
                            ore: current_mineral.ore - add_r_ore * self.blueprint.ore_cost_in_ore,
                            clay: current_mineral.clay,
                            obsi: current_mineral.obsi,
                            geo: current_mineral.geo,
                        };

                        if (add_r_ore + add_r_clay + add_r_obsi + add_r_geo) <= 1 {
                            ret.push(FullResourceList {
                                mineral: ResourceList {
                                    ore: current_mineral.ore + current.robot.ore,
                                    clay: current_mineral.clay + current.robot.clay,
                                    obsi: current_mineral.obsi + current.robot.obsi,
                                    geo: current_mineral.geo + current.robot.geo,
                                },
                                robot: ResourceList {
                                    ore: current.robot.ore + add_r_ore,
                                    clay: current.robot.clay + add_r_clay,
                                    obsi: current.robot.obsi + add_r_obsi,
                                    geo: current.robot.geo + add_r_geo,
                                }
                            });
                        }
                    }
                }
            }
        }

        ret
    }
}

mod parse {
    use crate::days::parse::*;
    use crate::days::parse::nom_goes_brrr::*;
    use nom::Parser;
    use super::Blueprint;

    pub(super) fn parse_and_collect(input: &str) -> super::Result<Vec<Blueprint>>
    {
        let parse_blueprint = tuple((
            preceded(
                tag("Blueprint ").and(parse_int::<u8, _>).and(tag(": Each ore robot costs ")),
                parse_int
            ),
            preceded(
                tag(" ore. Each clay robot costs "),
                parse_int
            ),
            preceded(
                tag(" ore. Each obsidian robot costs "),
                parse_int
            ),
            preceded(
                tag(" ore and "),
                parse_int
            ),
            preceded(
                tag(" clay. Each geode robot costs "),
                parse_int
            ),
            delimited(
                tag(" ore and "),
                parse_int,
                tag(" obsidian.")
            ),
        )).map(
            |x: (u8, u8, u8, u8, u8, u8)| Blueprint {
                ore_cost_in_ore: x.0,
                clay_cost_in_ore: x.1,
                obsi_cost_in_ore: x.2,
                geo_cost_in_ore: x.4,
                obsi_cost_in_clay: x.3,
                geo_cost_in_obsi: x.5
            }
        );

        let (_, ret) = make_verbose_error_message(input,
            many0(
                terminated(parse_blueprint, newline)
            )(input)
        )?;
        Ok(ret)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_blueprint_1() {
        let blueprint = Blueprint {
            ore_cost_in_ore: 4,
            clay_cost_in_ore: 2,
            obsi_cost_in_ore: 3,
            geo_cost_in_ore: 2,
            obsi_cost_in_clay: 14,
            geo_cost_in_obsi: 7,
        };
        let mut solv = BBSolv::new(blueprint, 24);
        solv.solve();
        solv.print_best();
        assert_eq!(solv.lower_bound, 9);
    }

    #[test]
    fn test_example_blueprint_2() {
        let blueprint = Blueprint {
            ore_cost_in_ore: 2,
            clay_cost_in_ore: 3,
            obsi_cost_in_ore: 3,
            geo_cost_in_ore: 3,
            obsi_cost_in_clay: 8,
            geo_cost_in_obsi: 12,
        };
        let mut solv = BBSolv::new(blueprint, 24);
        solv.solve();
        // solv.print_best();
        assert_eq!(solv.lower_bound, 12);
    }
}