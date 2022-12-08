use crate::days::internal_common::*;

#[derive(Debug)]
struct Grid {
    data: Vec<u32>,
    width: usize,
    height: usize
}

#[derive(Debug, Clone)]
struct Node {
    id: usize,
    tree_height: usize
}

impl Grid {
    fn from_input<Input>(input: &mut Input) -> Result<Self>
    where Input: Read
    {
        let mut data: Vec<u32> = Vec::new();
        let mut width = 0;
        let mut line_idx = 0;
        do_for_each_line(input, |line| {
            if line_idx == 0 {
                width = line.len();
            }
            else if width != line.len() {
                return Err(Error::new_token(0, line.len()));
            }
            line_idx += 1;
            for c in line.chars() {
                let tree_height = c.to_digit(10).ok_or(Error::new_token(0, line.len()))?;
                data.push(tree_height);
            }
            Ok(())
        })?;
        let height = line_idx;
        Ok(Self {
            data,
            width,
            height
        })
    }

    fn get_node_from_id(&self, id: usize) -> Node
    {
        Node {
            id: id,
            tree_height: self.data[id] as usize
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

    fn is_node_visible_from_outside(&self, node: &Node) -> bool
    {
        if self.is_node_visible_from_direction(node, |node| self.get_node_left(node)) {
            return true;
        }
        if self.is_node_visible_from_direction(node, |node| self.get_node_right(node)) {
            return true;
        }
        if self.is_node_visible_from_direction(node, |node| self.get_node_up(node)) {
            return true;
        }
        if self.is_node_visible_from_direction(node, |node| self.get_node_down(node)) {
            return true;
        }

        false
    }

    fn node_scenic_score(&self, node: &Node) -> usize
    {
        self.node_viewing_dist_in_direction(node, |node| self.get_node_left(node))
        * self.node_viewing_dist_in_direction(node, |node| self.get_node_right(node))
        * self.node_viewing_dist_in_direction(node, |node| self.get_node_up(node))
        * self.node_viewing_dist_in_direction(node, |node| self.get_node_down(node))
    }

    fn is_node_visible_from_direction<F>(&self, node: &Node, dir_func: F) -> bool
    where F: Fn(&Node)-> Option<Node>
    {
        let mut next_in_dir_maybe = dir_func(node);

        while next_in_dir_maybe.is_some() {
            let next_in_dir = next_in_dir_maybe.unwrap();

            if next_in_dir.tree_height < node.tree_height {
                next_in_dir_maybe = dir_func(&next_in_dir);
            }
            else {
                return false;
            }
        }

        true
    }

    fn node_viewing_dist_in_direction<F>(&self, node: &Node, dir_func: F) -> usize
    where F: Fn(&Node)-> Option<Node>
    {
        let mut viewing_dist = 0;
        let mut next_in_dir_maybe = dir_func(node);

        while next_in_dir_maybe.is_some() {
            viewing_dist += 1;
            let next_in_dir = next_in_dir_maybe.unwrap();

            if next_in_dir.tree_height < node.tree_height {
                next_in_dir_maybe = dir_func(&next_in_dir);
            }
            else {
                return viewing_dist;
            }
        }

        viewing_dist
    }
}

pub fn day_8_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let grid = Grid::from_input(input)?;

    let sum = (0..grid.data.len()).filter(|&node_id| {
        let node = grid.get_node_from_id(node_id);
        grid.is_node_visible_from_outside(&node)
    }).count();

    println!("Number of visible trees from outside is {}", sum);

    Ok(())
}

pub fn day_8_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let grid = Grid::from_input(input)?;

    let scenic_scores = (0..grid.data.len()).map(|node_id| {
        let node = grid.get_node_from_id(node_id);
        grid.node_scenic_score(&node)
    });

    let mut best_node = 0;
    let mut best_score = 0;
    for (node_id, score) in scenic_scores.enumerate() {
        if score > best_score {
            best_score = score;
            best_node = node_id;
        }
    }

    println!("Best scenic score is {} (best node {})", best_score, best_node);

    Ok(())
}