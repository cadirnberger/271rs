use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    data: String,
    next: Vec<String>,  // outgoing edges (prereq → dependent)
}

type Graph = HashMap<String, Node>;

/// Build a reverse adjacency map so we can compute depth.
fn compute_parents(graph: &Graph) -> HashMap<String, Vec<String>> {
    let mut parents: HashMap<String, Vec<String>> = HashMap::new();

    for (key, node) in graph {
        for next in &node.next {
            parents.entry(next.clone())
                   .or_default()
                   .push(key.clone());
        }
    }

    parents
}

/// Depth = longest path backward to a root (node with no parents)
fn depth(
    node: &str,
    graph: &Graph,
    parents: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, i32>,
) -> i32 {
    if let Some(&cached) = memo.get(node) {
        return cached;
    }

    let result = if let Some(pars) = parents.get(node) {
        1 + pars.iter()
                .map(|p| depth(p, graph, parents, memo))
                .max()
                .unwrap_or(0)
    } else {
        0
    };

    memo.insert(node.to_string(), result);
    result
}

/// Height = longest path forward to a leaf (node with no children)
fn height(node: &str, graph: &Graph, memo: &mut HashMap<String, i32>) -> i32 {
    if let Some(&cached) = memo.get(node) {
        return cached;
    }

    let result = if let Some(n) = graph.get(node) {
        if n.next.is_empty() {
            0 // leaf
        } else {
            1 + n.next
                    .iter()
                    .map(|child| height(child, graph, memo))
                    .max()
                    .unwrap()
        }
    } else {
        0
    };

    memo.insert(node.to_string(), result);
    result
}

fn main() {
    let stdin = io::stdin();
    let mut graph: Graph = HashMap::new();

    // Read lines like: C151:C152
    for line in stdin.lock().lines() {
        let l = line.unwrap();

        // Stop on empty line
        if l.trim().is_empty() {
            break;
        }

        // TRIM to remove Windows \r characters
        let clean = l.trim();

        let mut parts = clean.split(':');
        let a = parts.next().unwrap().to_string();
        let b = parts.next().unwrap().to_string();

        // Insert nodes if missing
        graph.entry(a.clone()).or_insert(Node {
            data: a.clone(),
            next: vec![],
        });

        graph.entry(b.clone()).or_insert(Node {
            data: b.clone(),
            next: vec![],
        });

        // Add edge: a -> b
        graph.get_mut(&a).unwrap().next.push(b.clone());
    }


    // Build reverse edges
    let parents = compute_parents(&graph);

    // Memo tables
    let mut depth_memo = HashMap::new();
    let mut height_memo = HashMap::new();

    // Required output
    println!("depth(C152) = {}", depth("C152", &graph, &parents, &mut depth_memo));
    println!("depth(C371) = {}", depth("C371", &graph, &parents, &mut depth_memo));

    println!("height(C152) = {}", height("C152", &graph, &mut height_memo));
    println!("height(C371) = {}", height("C371", &graph, &mut height_memo));
}

