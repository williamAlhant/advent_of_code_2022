use macro_lib::Grid2D;

trait Grid2D: Grid2DTypes {
    fn get_node_from_id(&self, id: usize) -> Self::Node;
    fn get_node_left(&self, current: &Self::Node) -> Option<Self::Node>;
    fn get_node_right(&self, current: &Self::Node) -> Option<Self::Node>;
    fn get_node_up(&self, current: &Self::Node) -> Option<Self::Node>;
    fn get_node_down(&self, current: &Self::Node) -> Option<Self::Node>;
}
trait Grid2DTypes {
    type DataType;
    type Node;
}

#[derive(Grid2D)]
struct Grid {
    data: Vec<u32>,
    height: usize,
    width: usize
}
#[derive(PartialEq, Debug)]
struct Node {
    id: usize,
    data: u32
}
impl Grid2DTypes for Grid {
    type DataType = u32;
    type Node = Node;
}

#[test]
fn kek()
{
    let grid = Grid {
        data: vec![1,2,3,4,5,6,7,8,9],
        height: 3,
        width: 3
    };
    assert_eq!(grid.get_node_from_id(0), Node { id: 0, data: 1 });
    assert_eq!(grid.get_node_left(&grid.get_node_from_id(0)), None);
    assert_eq!(grid.get_node_left(&grid.get_node_from_id(1)), Some(Node { id: 0, data: 1 }));

    let center_node = grid.get_node_from_id(4);
    assert_eq!(grid.get_node_left(&center_node), Some(Node { id: 3, data: 3 + 1 }));
    assert_eq!(grid.get_node_right(&center_node), Some(Node { id: 5, data: 5 + 1 }));
    assert_eq!(grid.get_node_up(&center_node), Some(Node { id: 1, data: 1 + 1 }));
    assert_eq!(grid.get_node_down(&center_node), Some(Node { id: 7, data: 7 + 1 }));
}