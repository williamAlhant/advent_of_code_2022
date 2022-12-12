use crate::days::internal_common::*;

#[derive(Debug)]
struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
    start: Node,
    end: Node
}

#[derive(Debug, Clone)]
struct Node {
    id: usize,
    data: u8
}

impl Grid {
    fn from_input<Input>(input: &mut Input) -> Result<Self>
    where Input: Read
    {
        let mut data: Vec<u8> = Vec::new();
        let mut width = 0;
        let mut line_idx = 0;
        let mut start = Node {id: 0, data: 0};
        let mut end = Node {id: 0, data: 0};
        do_for_each_line(input, |line| {
            if line_idx == 0 {
                width = line.len();
            }
            else if width != line.len() {
                return Err(Error::new_token(0, line.len()));
            }
            line_idx += 1;
            for c in line.bytes() {
                let elevation;
                if (b'a'..=b'z').contains(&c) {
                    elevation = c;
                }
                else if c == b'S' {
                    elevation = b'a';
                    start.id = data.len();
                    start.data = elevation;
                }
                else if c == b'E' {
                    elevation = b'z';
                    end.id = data.len();
                    end.data = elevation;
                }
                else {
                    return Err(Error::new_token(0, line.len()));
                }
                data.push(elevation);
            }
            Ok(())
        })?;
        let height = line_idx;
        Ok(Self {
            data,
            width,
            height,
            start,
            end
        })
    }

    fn get_node_from_id(&self, id: usize) -> Node
    {
        Node {
            id: id,
            data: self.data[id]
        }
    }

    fn get_node_left(&self, current: &Node) -> Option<Node>
    {
        let column = current.id % self.width;
        if column == 0 {
            return None;
        }
        let dest_id = current.id - 1;
        Some(self.get_node_from_id(dest_id))
    }

    fn get_node_right(&self, current: &Node) -> Option<Node>
    {
        let column = current.id % self.width;
        if column == self.width - 1 {
            return None;
        }
        let dest_id = current.id + 1;
        Some(self.get_node_from_id(dest_id))
    }

    fn get_node_up(&self, current: &Node) -> Option<Node>
    {
        let row = current.id / self.width;
        if row == 0 {
            return None;
        }
        let dest_id = current.id - self.width;
        Some(self.get_node_from_id(dest_id))
    }

    fn get_node_down(&self, current: &Node) -> Option<Node>
    {
        let row = current.id / self.width;
        if row == self.height - 1 {
            return None;
        }
        let dest_id = current.id + self.width;
        Some(self.get_node_from_id(dest_id))
    }

}

fn compute_cost_recurs(current: &Node, grid: &Grid, costs: &mut Vec<usize>)
{
    let node_accessible_from_current = |node: &Node, current: &Node| {
        current.data <= node.data + 1
    };

    let mut next_candidates: Vec<Node> = Vec::with_capacity(4);
    if let Some(node) = grid.get_node_left(current) {
        if node_accessible_from_current(&node, current) {
            next_candidates.push(node);
        }
    }
    if let Some(node) = grid.get_node_right(current) {
        if node_accessible_from_current(&node, current) {
            next_candidates.push(node);
        }
    }
    if let Some(node) = grid.get_node_up(current) {
        if node_accessible_from_current(&node, current) {
            next_candidates.push(node);
        }
    }
    if let Some(node) = grid.get_node_down(current) {
        if node_accessible_from_current(&node, current) {
            next_candidates.push(node);
        }
    }

    if next_candidates.len() == 0 {
        return;
    }

    for candidate in next_candidates {
        let new_cost = costs[current.id] + 1;
        if new_cost < costs[candidate.id] {
            costs[candidate.id] = new_cost;
            compute_cost_recurs(&candidate, grid, costs);
        }
    }

}

pub fn day_12_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let grid = Grid::from_input(input)?;

    let mut costs: Vec<usize> = vec![usize::MAX; grid.data.len()];
    costs[grid.end.id] = 0;
    compute_cost_recurs(&grid.end, &grid, &mut costs);

    println!("Cost is {}", costs[grid.start.id]);

    Ok(())
}

pub fn day_12_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let grid = Grid::from_input(input)?;

    let mut costs: Vec<usize> = vec![usize::MAX; grid.data.len()];
    costs[grid.end.id] = 0;
    compute_cost_recurs(&grid.end, &grid, &mut costs);

    let costs_with_idx_with_elevation_a: Vec<(usize, usize)> =
        costs.into_iter().enumerate().filter(|&(node_idx, _)| grid.data[node_idx] == b'a').collect();
    let min_cost = costs_with_idx_with_elevation_a.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().1;
    println!("Cost is {}", min_cost);

    Ok(())
}