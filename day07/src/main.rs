use petgraph::graphmap::GraphMap;
use petgraph::visit::Dfs;
use petgraph::EdgeType;
use regex::Regex;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    println!(
        "{} bag colors can contain a shiny gold bag",
        count_bags_which_contain_shiny_gold_bag(&input)
    );

    println!(
        "A shiny gold bag contains {} other bags",
        count_bags_inside_shiny_gold_bag(&input)
    );
}

// Given a line like this:
//
//     light red bags contain 1 bright white bag, 2 muted yellow bags
//
// Make a graph where "bright white" points to "light red", "muted yellow" points to "light red",
// then do a DFS from "shiny gold" and count the nodes encountered along the way.
fn count_bags_which_contain_shiny_gold_bag(input: &str) -> usize {
    let inner_colors_regex = Regex::new(r"(?P<count>\d) (?P<color>\w+ \w+)").unwrap();
    let edges: Vec<(&str, &str)> = input
        .lines()
        .flat_map(|line| {
            if let [outer_color, inner_colors] =
                line.split(" bags contain ").collect::<Vec<_>>()[..]
            {
                inner_colors_regex
                    .captures_iter(inner_colors)
                    .map(move |caps| {
                        let inner_color = caps.name("color").unwrap().as_str();
                        (inner_color, outer_color)
                    })
            } else {
                panic!("Invalid line");
            }
        })
        .collect();
    let graph = GraphMap::<&str, (), petgraph::Directed>::from_edges(edges);

    let mut visited_nodes = HashSet::new();
    let mut dfs = Dfs::new(&graph, "shiny gold");
    dfs.next(&graph); // Skip first node as we start with shiny gold and we don't want to count it.

    while let Some(nx) = dfs.next(&graph) {
        visited_nodes.insert(nx);
    }

    visited_nodes.len()
}

// Given a line like this:
//
//     light red bags contain 1 bright white bag, 2 muted yellow bags
//
// Make a graph where "light red" points to "bright white" as well as "muted yellow", then walk
// the graph from "shiny gold", counting bags along the way.
fn count_bags_inside_shiny_gold_bag(input: &str) -> u32 {
    let inner_colors_regex = Regex::new(r"(?P<count>\d) (?P<color>\w+ \w+)").unwrap();
    let edges: Vec<(&str, &str, u32)> = input
        .lines()
        .flat_map(|line| {
            if let [outer_color, inner_colors] =
                line.split(" bags contain ").collect::<Vec<_>>()[..]
            {
                inner_colors_regex
                    .captures_iter(inner_colors)
                    .map(move |caps| {
                        let inner_color = caps.name("color").unwrap().as_str();
                        let count = caps.name("count").unwrap().as_str().parse().unwrap();

                        (outer_color, inner_color, count)
                    })
            } else {
                panic!("Invalid line");
            }
        })
        .collect();
    let graph = GraphMap::<&str, u32, petgraph::Directed>::from_edges(edges);

    count_bags_inside(&graph, "shiny gold")
}

// It probably could use tail call optimization of some sort, but it doesn't crumble under the
// input string. ¯\_(ツ)_/¯
fn count_bags_inside<Ty: EdgeType>(graph: &GraphMap<&str, u32, Ty>, outer_color: &str) -> u32 {
    graph
        .edges(outer_color)
        .map(|(_, inner_color, count)| count + count * count_bags_inside(graph, inner_color))
        .sum()
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
    fn correctly_counts_bags_which_contain_shiny_gold_bag() {
        assert_eq!(4, count_bags_which_contain_shiny_gold_bag(INPUT));
    }

    #[test]
    fn correctly_counts_bags_inside_shiny_gold_bag_with_first_input() {
        assert_eq!(32, count_bags_inside_shiny_gold_bag(INPUT));
    }

    #[test]
    fn correctly_counts_bags_inside_shiny_gold_bag_with_second_input() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(126, count_bags_inside_shiny_gold_bag(input));
    }
}
