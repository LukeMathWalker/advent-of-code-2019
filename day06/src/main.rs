use petgraph::algo::{astar, is_cyclic_directed};
use petgraph::graphmap::{DiGraphMap, GraphMap};
use petgraph::{Direction, Undirected};
use std::io::BufRead;

fn read_input(path: &str) -> Vec<(String, String)> {
    fn parse(l: &str) -> (String, String) {
        let parts: Vec<_> = l.split(')').collect();
        (parts[0].into(), parts[1].into())
    }

    let file = std::fs::File::open(path).expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            let line = l.expect("Failed to read line");
            parse(&line)
        })
        .collect()
}

fn count_orbits<'a>(graph: &DiGraphMap<&'a str, ()>, root_node: &'a str) -> usize {
    let mut acc = 0;
    for neighbour in graph.neighbors_directed(root_node, Direction::Outgoing) {
        acc += _count_orbits(&graph, &neighbour, 1);
    }
    acc
}

fn _count_orbits<'a>(graph: &DiGraphMap<&'a str, ()>, node: &'a str, depth: usize) -> usize {
    let mut acc = depth;
    for neighbour in graph.neighbors_directed(node, Direction::Outgoing) {
        acc += _count_orbits(&graph, &neighbour, depth + 1);
    }
    acc
}

fn main() {
    let orbits = read_input("input.txt");
    let edges = orbits.iter().map(|(x, y)| (x.as_str(), y.as_str()));
    let graph: DiGraphMap<_, ()> = DiGraphMap::from_edges(edges.clone());

    assert!(!is_cyclic_directed(&graph));
    let n_orbits = count_orbits(&graph, "COM");
    println!("Total number of orbits: {:?}.", n_orbits);

    let graph: GraphMap<_, (), Undirected> = GraphMap::from_edges(edges);
    let (distance, _) =
        astar(&graph, "YOU", |f| f == "SAN", |_| 1, |_| 0).expect("Failed to find a path.");
    println!("Distance between YOU and SAN: {:?}", distance - 2);
}

#[cfg(test)]
mod tests {
    use crate::{count_orbits, parse};
    use petgraph::graphmap::DiGraphMap;

    #[test]
    fn part_1_example() {
        let edges = vec![
            ("COM", "B"),
            ("B", "G"),
            ("G", "H"),
            ("B", "C"),
            ("C", "D"),
            ("D", "I"),
            ("D", "E"),
            ("E", "F"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
        ];
        let graph: DiGraphMap<_, ()> = DiGraphMap::from_edges(&edges);
        let n_orbits = count_orbits(&graph, "COM");
        assert_eq!(n_orbits, 42);
    }
}
