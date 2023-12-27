use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

pub fn run(input: &str) -> usize {
    let graph = parse_graph(input);
    let mut edges = Vec::new();
    for (node, neighbors) in &graph {
        for neighbor in neighbors {
            if *node < *neighbor {
                edges.push((*node, *neighbor));
            }
        }
    }

    let found = Arc::new(AtomicBool::new(false));
    let result = Arc::new(AtomicUsize::new(0));

    (0..100000)
        .into_par_iter()
        .map_init(
            || UnionFind::new(graph.len()),
            |uf, _| {
                if found.load(Ordering::SeqCst) {
                    return None;
                }

                let (min_cut, size) = karger_min_cut(&mut uf.clone(), &edges);

                if min_cut == 3 {
                    found.store(true, Ordering::SeqCst);
                    let value = size;
                    let current_result = result.load(Ordering::SeqCst);
                    if value < current_result || current_result == 0 {
                        result.store(value, Ordering::SeqCst);
                    }
                    Some(value)
                } else {
                    None
                }
            },
        )
        .filter_map(|x| x)
        .for_each(|_| {});

    let result = result.load(Ordering::SeqCst);
    result * (graph.len() - result)
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

fn karger_min_cut(uf: &mut UnionFind, edges: &Vec<(usize, usize)>) -> (usize, usize) {
    contract(uf, edges, 2);

    let min_cut = edges
        .iter()
        .filter(|(a, b)| uf.find(*a) != uf.find(*b))
        .count();

    let root = uf.find(0);
    let size = uf.sizes[root];
    (min_cut, size)
}

fn contract(uf: &mut UnionFind, edges: &Vec<(usize, usize)>, num_clusters: usize) {
    while uf.num_clusters > num_clusters {
        let (node, neighbor) = edges[thread_rng().gen_range(0..edges.len())];
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
