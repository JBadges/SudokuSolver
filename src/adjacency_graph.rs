use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::sudoku_grid::SudokuGrid;

pub struct AdjacencyGraph {
    pub edges: HashMap<(usize, usize, usize), HashSet<(usize, usize, usize)>>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum BiColor {
    Red,
    Blue,
    None,
}

impl AdjacencyGraph {
    // Creates a new empty graph.
    pub fn new() -> Self {
        AdjacencyGraph {
            edges: HashMap::new(),
        }
    }

    // Adds an edge between two nodes. Nodes are added implicitly.
    pub fn add_edge(&mut self, from: (usize, usize, usize), to: (usize, usize, usize)) {
        self.edges.entry(from).or_insert(HashSet::new()).insert(to);
        self.edges.entry(to).or_insert(HashSet::new()).insert(from);
    }

    // Returns a list of neighbors for a given node.
    pub fn neighbors(&self, node: (usize, usize, usize)) -> Option<&HashSet<(usize, usize, usize)>> {
        self.edges.get(&node)
    }

    pub fn nodes(&self) -> Vec<(usize, usize, usize)> {
        self.edges.keys().cloned().collect()
    }

    pub fn remove_cell(&mut self, node: (usize, usize, usize)) -> Option<HashSet<(usize, usize, usize)>> {
        self.edges.remove(&node)
    }

    pub fn get_first(&mut self) -> Option<((usize, usize, usize), HashSet<(usize, usize, usize)>)> {
        self.edges.drain().next()
    }

    pub fn is_empty(&self) -> bool {
        return self.edges.is_empty();
    }

    pub fn merge_on_bivalue(&mut self, mut other: AdjacencyGraph, sgrid: &SudokuGrid) {
        let self_bivalued_nodes: Vec<(usize, usize, usize)> = self.nodes()
            .into_iter()
            .filter(|&(row, col, _)| sgrid.candidates[row][col].len() == 2)
            .collect();
        let other_bivalued_nodes: Vec<(usize, usize, usize)> = other.nodes()
        .into_iter()
        .filter(|&(row, col, _)| sgrid.candidates[row][col].len() == 2)
        .collect();

        for &self_node in &self_bivalued_nodes {
            for &other_node in &other_bivalued_nodes {
                let (self_row, self_col, self_num) = self_node;
                let (other_row, other_col, other_num) = other_node;

                if (self_row, self_col) != (other_row, other_col) { continue; }
                if self_num == other_num { continue; }

                self.add_edge(self_node, other_node);
            }
        }

        for (node, neighbors) in other.edges.drain() {
            self.edges.entry(node).or_insert_with(HashSet::new).extend(neighbors);
        }

        // Hacky way to include an edge between bivalued cell with no conjugate pair
        for &(row, col, num) in &self_bivalued_nodes {
            if sgrid.candidates[row][col].len() == 2 {
                let mut map_iter = sgrid.candidates[row][col].iter();

                let &value1 = map_iter.next().unwrap();
                let &value2 = map_iter.next().unwrap();

                self.edges.entry((row, col, value1)).or_insert_with(HashSet::new).insert((row, col, value2));
                self.edges.entry((row, col, value2)).or_insert_with(HashSet::new).insert((row, col, value1));
            }
        }
    }

    pub fn bicolor_graphs(graph: &AdjacencyGraph) -> Vec<HashMap<(usize, usize, usize), BiColor>> {
        let mut ret:Vec<HashMap<(usize, usize, usize), BiColor>> = Vec::new();
        for node in graph.nodes() {
            let mut found = false;
            for colored_graph in &ret {
                if colored_graph.contains_key(&node) {
                    found = true;
                    break;
                }
            }
            if !found {
                ret.push(AdjacencyGraph::bicolor_graph(graph, &node));
            }
        }

        return ret;
    }

    fn bicolor_graph(graph: &AdjacencyGraph, start_node: &(usize, usize, usize)) -> HashMap<(usize, usize, usize), BiColor> {
        let mut colors: HashMap<(usize, usize, usize), BiColor> = HashMap::new();
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();

        // Start with an arbitrary node
        queue.push_back(*start_node);
        colors.insert(*start_node, BiColor::Red);

        while let Some(node) = queue.pop_front() {
            let current_color = colors[&node];
            let opposite_color = match current_color {
                BiColor::Red => BiColor::Blue,
                BiColor::Blue => BiColor::Red,
                BiColor::None => panic!("Uncolored node found in queue!"),
            };

            if let Some(neighbors) = graph.neighbors(node) {
                for &neighbor in neighbors {
                    if !colors.contains_key(&neighbor) || colors[&neighbor] == BiColor::None {
                        colors.insert(neighbor, opposite_color);
                        queue.push_back(neighbor);
                    } else if colors[&neighbor] == current_color {
                        panic!("Graph is not bipartite!");
                    }
                }
            }
        }

        colors
    }
}
