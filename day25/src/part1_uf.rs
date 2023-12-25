use rand::Rng;
use std::collections::HashMap;

pub fn run(input: &str) -> usize {
    let graph = parse_graph(input);
    // println!("graph: {:?}", graph);
    loop {
        let (min_cut, size) = karger_min_cut(&graph);
        if min_cut == 3 {
            return (graph.len() - size) * size;
        }
    }
}

pub struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            sizes: vec![1; size],
            parents: (0..size).collect(),
        }
    }

    pub fn find(&mut self, key: usize) -> usize {
        if self.parents[key] == key {
            return key;
        }

        let parent = self.find(self.parents[key]);
        self.parents[key] = parent;
        parent
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let x = self.find(a);
        let y = self.find(b);

        // A and B are already in the same set -> nothing to do
        if x == y {
            return false;
        }

        let x_size = self.sizes[x];
        let y_size = self.sizes[y];

        if x_size >= y_size {
            self.sizes[x] += y_size;
            self.parents[y] = x;
        } else {
            self.sizes[y] += x_size;
            self.parents[x] = y;
        }

        true
    }
}

fn karger_min_cut(graph: &HashMap<usize, Vec<usize>>) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let mut uf = UnionFind::new(graph.len());
    let mut edges = Vec::new();
    for (node, neighbors) in graph {
        for neighbor in neighbors {
            if *node < *neighbor {
                edges.push((*node, *neighbor));
            }
        }
    }

    contract(&mut uf, &mut edges, 2, &mut rng);

    let min_cut = edges
        .iter()
        .filter(|(a, b)| uf.find(*a) != uf.find(*b))
        .count();

    let root = uf.find(0);
    let size = uf.sizes[root];

    (min_cut, size)
}

fn contract(
    uf: &mut UnionFind,
    edges: &mut Vec<(usize, usize)>,
    num_clusters: usize,
    rng: &mut rand::rngs::ThreadRng,
) {
    let mut n = uf.parents.len();
    // shuffle the edges
    for i in 0..edges.len() {
        let j = rng.gen_range(0..edges.len());
        edges.swap(i, j);
    }

    for (node, neighbor) in edges {
        if n <= num_clusters {
            break;
        }
        let node = uf.find(*node);
        let neighbor = uf.find(*neighbor);
        if node != neighbor {
            uf.union(node, neighbor);
            n -= 1;
        }
    }
}

fn parse_graph(input: &str) -> HashMap<usize, Vec<usize>> {
    let mut graph = HashMap::new();
    let mut node_counter = 0;
    let mut node_map = HashMap::new();
    for line in input.lines() {
        let id = &line[..3];
        let connections: Vec<&str> = line[5..].split_whitespace().collect();
        let node_id = if let Some(&id) = node_map.get(id) {
            id
        } else {
            node_map.insert(id, node_counter);
            graph.insert(node_counter, Vec::new());
            node_counter += 1;
            node_counter - 1
        };

        for connection in connections {
            let connection_id = if let Some(&id) = node_map.get(connection) {
                id
            } else {
                node_map.insert(connection, node_counter);
                graph.insert(node_counter, Vec::new());
                node_counter += 1;
                node_counter - 1
            };
            graph.get_mut(&node_id).unwrap().push(connection_id);

            // add the reverse connections for each edge
            graph.get_mut(&connection_id).unwrap().push(node_id);
        }
    }
    graph
}
// for line in input.lines() {
//     let id = &line[..3];
//     let connections: HashSet<&str> = line[5..].split_whitespace().collect();
//     // check if the node already exists
//     if !graph.contains_key(id) {
//         graph.insert(id, connections.clone());
//     } else {
//         graph.get_mut(id).unwrap().extend(connections.clone());
//     }

//     // add the reverse connections for each edge
//     for connection in connections {
//         if !graph.contains_key(connection) {
//             let mut set = HashSet::new();
//             set.insert(id);
//             graph.insert(connection, set);
//         } else {
//             graph.get_mut(connection).unwrap().insert(id);
//         }
//     }
// }
// graph
