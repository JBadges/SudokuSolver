use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;

pub struct AdjacencyGraph {
    edges: HashMap<(usize, usize), HashSet<(usize, usize)>>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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
    pub fn add_edge(&mut self, from: (usize, usize), to: (usize, usize)) {
        self.edges.entry(from).or_insert(HashSet::new()).insert(to);
        self.edges.entry(to).or_insert(HashSet::new()).insert(from);
    }

    // Returns a list of neighbors for a given node.
    pub fn neighbors(&self, node: (usize, usize)) -> Option<&HashSet<(usize, usize)>> {
        self.edges.get(&node)
    }

    pub fn nodes(&self) -> Vec<(usize, usize)> {
        self.edges.keys().cloned().collect()
    }

    pub fn remove_cell(&mut self, node: (usize, usize)) -> Option<HashSet<(usize, usize)>> {
        self.edges.remove(&node)
    }

    pub fn get_first(&mut self) -> Option<((usize, usize), HashSet<(usize, usize)>)> {
        self.edges.drain().next()
    }

    pub fn is_empty(&self) -> bool {
        return self.edges.is_empty();
    }

    pub fn bicolor_graphs(graph: &AdjacencyGraph) -> Vec<HashMap<(usize, usize), BiColor>> {
        let mut ret:Vec<HashMap<(usize, usize), BiColor>> = Vec::new();
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

    fn bicolor_graph(graph: &AdjacencyGraph, start_node: &(usize, usize)) -> HashMap<(usize, usize), BiColor> {
        let mut colors: HashMap<(usize, usize), BiColor> = HashMap::new();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

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
