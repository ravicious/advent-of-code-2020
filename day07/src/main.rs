use petgraph::graphmap::GraphMap;
use petgraph::visit::Dfs;
use regex::Regex;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    println!(
        "{} bag colors can contain a shiny gold bag",
        count_bags(&input)
    );
}

fn count_bags(input: &str) -> usize {
    let raw_inner_colors_regex = Regex::new(r"(?P<count>\d) (?P<color>\w+ \w+)").unwrap();
    let edges: Vec<(&str, &str)> = input
        .lines()
        .flat_map(|line| {
            if let [outer_color, raw_inner_colors] =
                line.split(" bags contain ").collect::<Vec<_>>()[..]
            {
                raw_inner_colors_regex
                    .captures_iter(raw_inner_colors)
                    .map(move |caps| (caps.name("color").unwrap().as_str(), outer_color))
            } else {
                panic!("Invalid line");
            }
        })
        .collect();
    let graph = GraphMap::<&str, (), petgraph::Directed>::from_edges(edges);

    let mut visited_nodes = HashSet::new();
    let mut dfs = Dfs::new(&graph, "shiny gold");
    dfs.next(&graph); // Skip first node as we start with shiny gold.

    while let Some(nx) = dfs.next(&graph) {
        visited_nodes.insert(nx);
    }

    visited_nodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn correctly_counts_bags() {
        assert_eq!(4, count_bags(INPUT));
    }
}
