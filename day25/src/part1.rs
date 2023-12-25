use petgraph::dot::{Config, Dot};
use petgraph::graph::Graph;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

pub fn run(input: &str) -> usize {
    let graph = parse_graph(input);

    // solve_brute_force(graph.clone())

    let nodes = graph.keys().cloned().collect::<HashSet<&str>>();
    let mut edges = HashSet::new();
    for (node, neighbors) in &graph {
        for neighbor in neighbors {
            edges.insert((*node, *neighbor));
        }
    }
    let mut cluster_counter = HashMap::new();
    for node in nodes.iter() {
        cluster_counter.insert(*node, 1);
    }
    let mut rng = rand::thread_rng();
    let mut min_diff = std::usize::MAX;
    let mut best_cut = (0, 0);
    for _ in 0..100 {
        let result = solve_randomized_search(
            nodes.clone(),
            edges.clone(),
            cluster_counter.clone(),
            &mut rng,
        );
        let diff = (result.0 as isize - result.1 as isize).abs() as usize;
        if diff < min_diff {
            min_diff = diff;
            best_cut = result;
        }
    }
    best_cut.0 * best_cut.1
}

fn solve_randomized_search<'a>(
    mut super_nodes: HashSet<&'a str>,
    mut super_edges: HashSet<(&'a str, &'a str)>,
    mut cluster_counter: HashMap<&'a str, usize>,
    rng: &mut rand::rngs::ThreadRng,
) -> (usize, usize) {
    while super_nodes.len() > 2 {
        let (node1, node2) = *super_edges
            .iter()
            .nth(rng.gen_range(0..super_edges.len()))
            .unwrap();

        merge_nodes(
            &mut super_nodes,
            &mut super_edges,
            &mut cluster_counter,
            node1,
            node2,
        );
    }

    // return the two super nodes cluster sizes
    let mut iter = super_nodes.iter();
    let node1 = iter.next().unwrap();
    let node2 = iter.next().unwrap();
    (cluster_counter[node1], cluster_counter[node2])
}

fn merge_nodes<'a>(
    super_nodes: &mut HashSet<&'a str>,
    super_edges: &mut HashSet<(&'a str, &'a str)>,
    cluster_counter: &mut HashMap<&'a str, usize>,
    node1: &'a str,
    node2: &'a str,
) {
    super_nodes.remove(node2);
    super_edges.remove(&(node1, node2));
    super_edges.remove(&(node2, node1));
    cluster_counter.insert(node1, cluster_counter[node1] + cluster_counter[node2]);
    for &node in super_nodes.iter() {
        if super_edges.contains(&(node, node2)) {
            super_edges.remove(&(node, node2));
            super_edges.remove(&(node2, node));
            if !super_edges.contains(&(node, node1)) {
                super_edges.insert((node, node1));
                super_edges.insert((node1, node));
            }
        }
    }
}

fn solve_brute_force(graph: HashMap<&str, HashSet<&str>>) -> usize {
    let mut edges: Vec<(&str, &str)> = Vec::new();
    for (node, neighbors) in &graph {
        for neighbor in neighbors {
            edges.push((*node, *neighbor));
        }
    }
    // sort edges by the number of connections
    edges.sort_by(|a, b| {
        let a_connections = graph.get(a.0).unwrap().len() + graph.get(a.1).unwrap().len();
        let b_connections = graph.get(b.0).unwrap().len() + graph.get(b.1).unwrap().len();
        a_connections.cmp(&b_connections)
    });

    let mut c;
    let mut c1 = 0;
    let mut c2 = 0;
    let mut visited = HashSet::new();
    'search: for i in 0..edges.len() {
        for j in i + 1..edges.len() {
            for k in j + 1..edges.len() {
                let mut graph_copy = graph.clone();
                graph_copy.get_mut(edges[i].0).unwrap().remove(edges[i].1);
                graph_copy.get_mut(edges[i].1).unwrap().remove(edges[i].0);
                graph_copy.get_mut(edges[j].0).unwrap().remove(edges[j].1);
                graph_copy.get_mut(edges[j].1).unwrap().remove(edges[j].0);
                graph_copy.get_mut(edges[k].0).unwrap().remove(edges[k].1);
                graph_copy.get_mut(edges[k].1).unwrap().remove(edges[k].0);

                let mut distinct_clusters = 0;
                c1 = 0;
                c2 = 0;
                // reset visited
                visited.clear();
                for node in graph_copy.keys() {
                    if !visited.contains(node) {
                        distinct_clusters += 1;
                        if distinct_clusters > 2 {
                            break;
                        }
                        c = get_cluster_size(&graph_copy, node, &mut visited);

                        if distinct_clusters == 1 {
                            c1 = c;
                        } else {
                            c2 = c;
                        }
                    }
                }

                if distinct_clusters == 2 {
                    break 'search;
                }
            }
        }
    }

    c1 * c2
}

fn visualize_graph(graph_map: &HashMap<&str, HashSet<&str>>) {
    let mut graph = Graph::new_undirected();
    let mut nodes = HashMap::new();

    for (node, _) in graph_map {
        let node_idx = graph.add_node(*node);
        nodes.insert(*node, node_idx);
    }

    for (node, edges) in graph_map {
        for edge in edges {
            if let Some(&edge_idx) = nodes.get(edge) {
                // only add the edge if it doesn't already exist
                if !graph.contains_edge(nodes[node], edge_idx) {
                    graph.add_edge(nodes[node], edge_idx, ());
                }
            }
        }
    }
    // export to DOT format
    let dot = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let mut file = File::create("graph.dot").unwrap();
    file.write_all(dot.as_bytes()).unwrap();
}

fn get_cluster_size<'a>(
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    node: &'a str,
    visited: &mut HashSet<&'a str>,
) -> usize {
    if visited.contains(node) {
        return 0;
    }

    visited.insert(node);
    let mut count = 1; // counting the current node

    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            count += get_cluster_size(graph, neighbor, visited);
        }
    }

    count
}

fn parse_graph(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let id = &line[..3];
        let connections: HashSet<&str> = line[5..].split_whitespace().collect();
        // check if the node already exists
        if !graph.contains_key(id) {
            graph.insert(id, connections.clone());
        } else {
            graph.get_mut(id).unwrap().extend(connections.clone());
        }

        // add the reverse connections for each edge
        for connection in connections {
            if !graph.contains_key(connection) {
                let mut set = HashSet::new();
                set.insert(id);
                graph.insert(connection, set);
            } else {
                graph.get_mut(connection).unwrap().insert(id);
            }
        }
    }
    graph
}
