use rand::Rng;
use std::collections::HashMap;

pub fn run(input: &str) -> usize {
    let graph = parse_graph(input);
    let mut rng = rand::thread_rng();
    let uf = UnionFind::new(graph.len());
    let mut edges = Vec::new();
    for (node, neighbors) in &graph {
        for neighbor in neighbors {
            if *node < *neighbor {
                edges.push((*node, *neighbor));
            }
        }
    }

    loop {
        let (min_cut, size) = karger_min_cut(&mut uf.clone(), &edges, &mut rng);
        if min_cut == 3 {
            return (graph.len() - size) * size;
        }
    }
}

#[derive(Clone)]
pub struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    num_clusters: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            sizes: vec![1; size],
            parents: (0..size).collect(),
            num_clusters: size,
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

    pub fn union(&mut self, a: usize, b: usize) {
        let x = self.find(a);
        let y = self.find(b);

        // A and B are already in the same set -> nothing to do
        if x == y {
            return;
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

        self.num_clusters -= 1;
    }
}

fn karger_min_cut(
    mut uf: &mut UnionFind,
    edges: &Vec<(usize, usize)>,
    mut rng: &mut rand::rngs::ThreadRng,
) -> (usize, usize) {
    // println!("cluster_count: {}", cluster_count);
    if uf.num_clusters < 6 {
        contract(&mut uf, &edges, 2, &mut rng);
    } else {
        let t = (1.0 + uf.num_clusters as f64 / 2.0).ceil() as usize;

        let mut uf_copy = uf.clone();

        contract(&mut uf, &edges, t, &mut rng);
        let (min_cut1, size1) = karger_min_cut(&mut uf, edges, &mut rng);
        if min_cut1 == 3 {
            return (min_cut1, size1);
        }

        contract(&mut uf_copy, &edges, t, &mut rng);
        let (min_cut2, size2) = karger_min_cut(&mut uf_copy, edges, &mut rng);

        if min_cut1 < min_cut2 {
            return (min_cut1, size1);
        } else {
            return (min_cut2, size2);
        }
    }

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
    edges: &Vec<(usize, usize)>,
    num_clusters: usize,
    rng: &mut rand::rngs::ThreadRng,
) {
    while uf.num_clusters > num_clusters {
        let (node, neighbor) = edges[rng.gen_range(0..edges.len())];
        uf.union(node, neighbor);
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
