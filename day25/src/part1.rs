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
    let mut edges = HashMap::new();
    for (node, neighbors) in &graph {
        for neighbor in neighbors {
            edges.insert((*node, *neighbor), 1usize);
        }
    }
    let mut cluster_counter = HashMap::new();
    for node in nodes.iter() {
        cluster_counter.insert(*node, 1);
    }
    let mut rng = rand::thread_rng();
    loop {
        let (cut, total_edges) = solve_randomized_search(
            nodes.clone(),
            edges.clone(),
            cluster_counter.clone(),
            &mut rng,
        );
        if total_edges == 6 {
            return cut.0 * cut.1;
        }
    }
}

fn solve_randomized_search<'a>(
    mut super_nodes: HashSet<&'a str>,
    mut super_edges: HashMap<(&'a str, &'a str), usize>,
    mut cluster_counter: HashMap<&'a str, usize>,
    rng: &mut rand::rngs::ThreadRng,
) -> ((usize, usize), usize) {
    if super_nodes.len() <= 6 {
        contract(
            &mut super_nodes,
            &mut super_edges,
            &mut cluster_counter,
            rng,
            2,
        );
    } else {
        let t = (1.0 + (super_nodes.len() as f64 / 2.0).ceil()) as usize;
        // make two clones of the graph
        let mut super_nodes1 = super_nodes.clone();
        let mut super_edges1 = super_edges.clone();
        let mut cluster_counter1 = cluster_counter.clone();
        let mut super_nodes2 = super_nodes.clone();
        let mut super_edges2 = super_edges.clone();
        let mut cluster_counter2 = cluster_counter.clone();

        // contract the first graph
        contract(
            &mut super_nodes1,
            &mut super_edges1,
            &mut cluster_counter1,
            rng,
            t,
        );
        // contract the second graph
        contract(
            &mut super_nodes2,
            &mut super_edges2,
            &mut cluster_counter2,
            rng,
            t,
        );

        // solve the two subproblems
        let result1 = solve_randomized_search(super_nodes1, super_edges1, cluster_counter1, rng);
        let result2 = solve_randomized_search(super_nodes2, super_edges2, cluster_counter2, rng);

        // return the best result (one with least difference)
        if result1.1 < result2.1 {
            return result1;
        } else {
            return result2;
        }
    }

    // sum of the weights of the edges between in the edges
    let mut sum = 0;
    for ((node1, node2), weight) in super_edges.iter() {
        if cluster_counter[node1] != cluster_counter[node2] {
            sum += weight;
        }
    }

    // the two super nodes cluster sizes
    let mut iter = super_nodes.iter();
    let node1 = iter.next().unwrap();
    let node2 = iter.next().unwrap();
    ((cluster_counter[node1], cluster_counter[node2]), sum)
}

fn contract<'a>(
    super_nodes: &mut HashSet<&'a str>,
    super_edges: &mut HashMap<(&'a str, &'a str), usize>,
    cluster_counter: &mut HashMap<&'a str, usize>,
    rng: &mut rand::rngs::ThreadRng,
    num_nodes: usize,
) {
    while super_nodes.len() > num_nodes {
        let (&(node1, node2), _) = super_edges
            .iter()
            .nth(rng.gen_range(0..super_edges.len()))
            .unwrap();

        merge_nodes(super_nodes, super_edges, cluster_counter, node1, node2);
    }
}

fn merge_nodes<'a>(
    super_nodes: &mut HashSet<&'a str>,
    super_edges: &mut HashMap<(&'a str, &'a str), usize>,
    cluster_counter: &mut HashMap<&'a str, usize>,
    node1: &'a str,
    node2: &'a str,
) {
    super_nodes.remove(node2);
    super_edges.remove(&(node1, node2)).unwrap();
    super_edges.remove(&(node2, node1));
    cluster_counter.insert(node1, cluster_counter[node1] + cluster_counter[node2]);
    for &node in super_nodes.iter() {
        if super_edges.contains_key(&(node, node2)) {
            let w1 = super_edges.remove(&(node, node2)).unwrap();
            super_edges.remove(&(node2, node));
            if !super_edges.contains_key(&(node, node1)) {
                super_edges.insert((node, node1), w1);
                super_edges.insert((node1, node), w1);
            } else {
                let w2 = *super_edges.get(&(node, node1)).unwrap();
                super_edges.insert((node, node1), w1 + w2);
                super_edges.insert((node1, node), w1 + w2);
            }
        }
    }
}

// doesn't work on a large input will take forever
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
