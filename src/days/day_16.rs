use crate::days::internal_common::*;
use std::collections::HashMap;
use rand::prelude::*;

fn get_structs_for_solv<Input>(input: &mut Input) -> Result<(Vec<Valve>, Vec<ValveForSolv>, ValveId)>
where Input: Read
{
    let input = get_whole_input_as_string(input)?;
    let mut valves = parse::parse_and_collect(&input)?;
    remap_valves_ids(&mut valves);

    let mut valves_for_solv: Vec<ValveForSolv> = Vec::new();
    for v in &valves {
        let distances;
        if v.flow_rate > 0 || v.name == "AA" {
            distances = compute_distances(v.id, &valves);
        } else {
            distances = Vec::new();
        }
        valves_for_solv.push(ValveForSolv {def: v.clone(), distances});
    }
    let initial_id = valves.iter().find(|v| v.name == "AA").unwrap().id;

    Ok((valves, valves_for_solv, initial_id))
}

pub fn day_16_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let (valves, valves_for_solv, initial_id) = get_structs_for_solv(input)?;

    let non_null_flow_valves: Vec<ValveId> = valves
        .iter()
        .filter(|v| v.flow_rate > 0)
        .map(|v| v.id)
        .collect();

    let mut visit_order_generator = VisitOrderGenerator::new(non_null_flow_valves);

    let annealing_temp = 2000.0;
    let annealing_temp_decrease_factor = 0.995;
    let annealing_final_temp = 2.0;
    let annealing_it_length = 100;
    let print_period = 100;
    let mut num_it = 0;
    let mut score = 0;
    let mut annealing = Annealing::new(annealing_temp, annealing_temp_decrease_factor, annealing_final_temp);

    while annealing.above_final_temp() {

        for _i in 0..annealing_it_length {
            visit_order_generator.swap();
            let next_score = get_score(30, initial_id, &visit_order_generator.current_visit_order, &valves_for_solv);
            if annealing.transition(score, next_score) {
                score = next_score;
            } else {
                visit_order_generator.cancel_swap();
            }
        }
        annealing.decrease_temp();
        num_it += 1;
        if num_it % print_period == 0 {
            println!("score: {}", score);
        }
    }

    dbg!(&visit_order_generator.current_visit_order, score, num_it);
    
    Ok(())
}

pub fn day_16_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let (valves, valves_for_solv, initial_id) = get_structs_for_solv(input)?;

    let non_null_flow_valves: Vec<ValveId> = valves
        .iter()
        .filter(|v| v.flow_rate > 0)
        .map(|v| v.id)
        .collect();

    let mut my_visit_order: Vec<ValveId> = non_null_flow_valves.clone();
    let mut el_visit_order: Vec<ValveId> = Vec::with_capacity(non_null_flow_valves.len());
    let mut my_visit_order_tmp: Vec<ValveId> = Vec::with_capacity(non_null_flow_valves.len());
    let mut el_visit_order_tmp: Vec<ValveId> = Vec::with_capacity(non_null_flow_valves.len());

    let annealing_temp = 2000.0;
    let annealing_temp_decrease_factor = 0.995;
    let annealing_final_temp = 2.0;
    let annealing_it_length = 100;
    let print_period = 100;
    let mut num_it = 0;
    let mut score = 0;
    let mut annealing = Annealing::new(annealing_temp, annealing_temp_decrease_factor, annealing_final_temp);

    let mut permut_rng = rand::rngs::StdRng::seed_from_u64(23456);

    while annealing.above_final_temp() {

        for _i in 0..annealing_it_length {

            my_visit_order_tmp.clone_from(&my_visit_order);
            el_visit_order_tmp.clone_from(&el_visit_order);
            if permut_rng.gen_bool(0.5) {
                let swap_in_my;
                if my_visit_order_tmp.len() < 2 {
                    swap_in_my = false;
                }
                else if el_visit_order_tmp.len() < 2 {
                    swap_in_my = true;
                }
                else {
                    swap_in_my = permut_rng.gen_bool(0.5);
                }
                if swap_in_my {
                    VisitOrderGenerator::swap_in_place(&mut permut_rng, &mut my_visit_order_tmp);
                }
                else {
                    VisitOrderGenerator::swap_in_place(&mut permut_rng, &mut el_visit_order_tmp);
                }
            }
            else {
                VisitOrderGenerator::move_between_two(&mut permut_rng, &mut my_visit_order_tmp, &mut el_visit_order_tmp);
            }

            let next_score = get_score_with_elephant(initial_id, &my_visit_order_tmp, &el_visit_order_tmp, &valves_for_solv);
            if annealing.transition(score, next_score) {
                score = next_score;
                my_visit_order.clone_from(&my_visit_order_tmp);
                el_visit_order.clone_from(&el_visit_order_tmp);
            }
        }

        annealing.decrease_temp();
        num_it += 1;
        if num_it % print_period == 0 {
            println!("score: {}", score);
        }
    }

    println!("{:?} {:?}", &my_visit_order, &el_visit_order);
    dbg!(score, num_it);
    
    Ok(())
}

struct Annealing {
    temp: f32,
    temp_decrease_factor: f32,
    final_temp: f32,
    transition_rng: rand::rngs::StdRng
}

impl Annealing {
    fn new(temp: f32, temp_decrease_factor: f32, final_temp: f32) -> Self {
        Self {
            temp,
            temp_decrease_factor,
            final_temp,
            transition_rng: rand::rngs::StdRng::seed_from_u64(54321)
        }
    }

    fn decrease_temp(&mut self) {
        self.temp *= self.temp_decrease_factor;
    }

    fn transition(&mut self, score: Score, next_score: Score) -> bool {
        if next_score > score {
            return true;
        }
        let p = ((next_score as f32 - score as f32) / self.temp).exp();
        let r = self.transition_rng.gen_range(0.0..1.0);
        r < p
    }

    fn above_final_temp(&self) -> bool {
        self.temp > self.final_temp
    }
}

struct VisitOrderGenerator {
    current_visit_order: Vec<ValveId>,
    buffer: Vec<ValveId>,
    rng: rand::rngs::StdRng
}

impl VisitOrderGenerator {
    fn new(current_visit_order: Vec<ValveId>) -> Self {
        let len = current_visit_order.len();
        Self {
            current_visit_order,
            buffer: vec![0; len],
            rng: rand::rngs::StdRng::seed_from_u64(23456)
        }
    }

    fn swap(&mut self) {
        let len = self.current_visit_order.len();

        self.buffer.copy_from_slice(&self.current_visit_order[0..len]);

        let mut a = self.rng.gen_range(0..len);
        let mut b = self.rng.gen_range(0..len);
        if a > b {
            (a, b) = (b, a);
        }
        for i in 0..(b - a + 1) {
            self.current_visit_order[a + i] = self.buffer[b - i];
        }
    }

    fn cancel_swap(&mut self) {
        let len = self.current_visit_order.len();
        self.current_visit_order.copy_from_slice(&self.buffer[0..len]);
    }

    fn swap_in_place(rng: &mut rand::rngs::StdRng, visit_order: &mut Vec<ValveId>) {
        let len = visit_order.len();
        let mut a = rng.gen_range(0..len);
        let mut b = rng.gen_range(0..len);
        if a > b {
            (a, b) = (b, a);
        }
        for i in 0..(b - a + 1) / 2 {
            visit_order.swap(a + i, b - i);
        }
    }

    fn move_between_two(rng: &mut rand::rngs::StdRng, visit_order_a: &mut Vec<ValveId>, visit_order_b: &mut Vec<ValveId>) {
        if visit_order_a.len() == 0 || (visit_order_b.len() != 0 && rng.gen_bool(0.5)) {
            assert_ne!(visit_order_b.len(), 0);
            Self::move_between_two(rng, visit_order_b, visit_order_a);
            return;
        }
        assert_ne!(visit_order_a.len(), 0);

        let t;
        if rng.gen_bool(0.5) {
            t = visit_order_a.pop().unwrap();
        }
        else {
            t = visit_order_a.remove(0);
        }
        if rng.gen_bool(0.5) {
            visit_order_b.push(t);
        }
        else {
            visit_order_b.insert(0, t);
        }
    }

}

type ValveId = u16;

#[derive(Debug, Clone)]
struct Valve {
    id: ValveId,
    name: String,
    flow_rate: u8,
    leads_to: Vec<ValveId>
}

#[derive(Debug)]
struct ValveForSolv {
    def: Valve,
    distances: Vec<u32>
}

fn compute_distances(origin: ValveId, valves: &Vec<Valve>) -> Vec<u32>
{
    let mut distances: Vec<u32> = vec![u32::MAX; valves.len()];
    distances[origin as usize] = 0;
    compute_distances_recurs(origin, valves, &mut distances);
    distances
}

fn compute_distances_recurs(current: ValveId, valves: &Vec<Valve>, costs: &mut Vec<u32>)
{
    for &candidate in &valves[current as usize].leads_to {
        let new_cost = costs[current as usize] + 1;
        if new_cost < costs[candidate as usize] {
            costs[candidate as usize] = new_cost;
            compute_distances_recurs(candidate, valves, costs);
        }
    }
}

fn get_score(time: Score, start_id: ValveId, visit_order: &Vec<ValveId>, valves: &Vec<ValveForSolv>) -> Score
{
    let mut time_left = time;
    let mut score: Score = 0;
    let mut current_valve = start_id;
    for &v_id in visit_order {
        let time_delta = (1 + valves[current_valve as usize].distances[v_id as usize]) as Score;
        if time_delta > time_left {
            return score;
        }
        time_left -= time_delta;
        current_valve = v_id;
        score += valves[current_valve as usize].def.flow_rate as Score * time_left;
    }
    score
}

fn get_score_with_elephant(start_id: ValveId,
    visit_order: &Vec<ValveId>, elephant_visit_order: &Vec<ValveId>,
    valves: &Vec<ValveForSolv>) -> Score
{
    get_score(26, start_id, visit_order, valves) + get_score(26, start_id, elephant_visit_order, valves)
}

type Score = u64;

fn remap_valves_ids(valves: &mut Vec<Valve>)
{
    let mut id_map: HashMap<ValveId, ValveId> = HashMap::new();
    for (idx, valve) in valves.iter_mut().enumerate() {
        id_map.insert(valve.id, idx as ValveId);
        valve.id = idx as u16;
    }
    for valve_source in valves {
        for i in 0..valve_source.leads_to.len() {
            valve_source.leads_to[i] = *id_map.get(&valve_source.leads_to[i]).unwrap();
        }
    }
}

mod parse {
    use crate::days::parse::*;
    use crate::days::parse::nom_goes_brrr::*;
    use nom::Parser;
    use super::Valve;

    pub(super) fn parse_and_collect(input: &str) -> super::Result<Vec<Valve>>
    {
        let shitty_tag_variants = alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve ")
        ));
        let parse_valve = preceded(tag("Valve "), take(2usize))
            .and(preceded(tag(" has flow rate="), parse_int))
            .and(preceded(shitty_tag_variants, separated_list0(tag(", "), take(2usize))))
            .map(|((name, flow_rate), leads_to): ((&str, u8), Vec<&str>)| {
                let name_to_id = |name: &str| {
                    let name_bytes = name.as_bytes();
                    ((name_bytes[0] as u16) << 8) + name_bytes[1] as u16
                };
                Valve {
                    id: name_to_id(name),
                    name: name.to_string(),
                    flow_rate,
                    leads_to: leads_to.into_iter().map(name_to_id).collect()
                }
            })
            ;

        let (_, valves) = make_verbose_error_message(input,
            many0(
                terminated(parse_valve, newline)
            )(input)
        )?;
        Ok(valves)
    }
}