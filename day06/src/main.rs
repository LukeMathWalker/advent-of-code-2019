use petgraph::graphmap::DiGraphMap;
use petgraph::Direction;
use std::io::BufRead;
use petgraph::algo::is_cyclic_directed;

fn read_input(path: &str) -> Vec<(String, String)> {
    let file = std::fs::File::open(path).expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            let line = l.expect("Failed to read line");
            let center = line[0..3].to_string();
            let orbiting = line[4..6].to_string();
            (center, orbiting)
        })
        .collect()
}

fn count_orbits(graph: &DiGraphMap<&str, ()>) -> usize {
    let n_direct_orbits = graph.edge_count();
    let n_indirect_orbits = count_indirect_orbits(graph, "COM");
    n_direct_orbits + n_indirect_orbits
}

fn count_indirect_orbits<'a>(graph: &DiGraphMap<&'a str, ()>, root_node: &'a str) -> usize {
    let mut acc = 0;
    for neighbour in graph.neighbors_directed(root_node, Direction::Outgoing) {
        acc += _count_indirect_orbits(&graph, &neighbour, 0);
    }
    acc
}

fn _count_indirect_orbits<'a>(graph: &DiGraphMap<&'a str, ()>, node: &'a str, depth: usize) -> usize {
    let mut acc = depth;
    for neighbour in graph.neighbors_directed(node, Direction::Outgoing) {
        acc += _count_indirect_orbits(&graph, &neighbour, depth + 1);
    }
    acc
}

fn main() {
    let orbits = read_input("input.txt");
    let edges = orbits.iter().map(|(x, y)| (x.as_str(), y.as_str()));
    let graph: DiGraphMap<_, ()> = DiGraphMap::from_edges(edges);

    assert!(!is_cyclic_directed(&graph));
    let n_orbits = count_orbits(&graph);
    println!("Total number of orbits: {:?}.", n_orbits);
}

#[cfg(test)]
mod tests {
    use petgraph::graphmap::DiGraphMap;
    use crate::count_orbits;

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
        let n_orbits = count_orbits(&graph);
        assert_eq!(n_orbits, 42);
    }
}
