use std::env;
use std::fs;
use std::cell::RefCell;
use std::rc::Rc;

// With gratitude to the timely https://eli.thegreenplace.net/2021/rust-data-structures-with-circular-references/


struct Node {
    name: String,
    connected: NodeList,
    visited: bool,
}

type NodeLink = Rc<RefCell<Node>>;

struct NodeList(Vec<NodeLink>);

impl NodeList {
    fn find_by_name(&mut self, name: &str) -> Option<NodeLink> {
        for node in &self.0 {
            if name == node.borrow().name {
                return Some(Rc::clone(&node));
            }
        }
        return None;
    }

    fn find_by_name_or_create(&mut self, name: &str) -> NodeLink {
        return match self.find_by_name(name) {
            Some(node) => Rc::clone(&node),
            None => {
                let node = Node{
                    name: String::from(name),
                    connected: NodeList(Vec::new()),
                    visited: false
                };
                let nodelink = Rc::new(RefCell::new(node));
                self.0.push(Rc::clone(&nodelink));
                return nodelink;
            }
        }
    }
}

// Returns the starting node
fn parse_edge_list(input: &str) -> NodeLink {
    let mut nodes = NodeList(Vec::new());
    for line in input.trim().split("\n") {
        let mut caves = line.trim().splitn(2, "-");
        let first = nodes.find_by_name_or_create(caves.next().unwrap());
        let second = nodes.find_by_name_or_create(caves.next().unwrap());
        first.borrow_mut().connected.0.push(Rc::clone(&second));
        second.borrow_mut().connected.0.push(first);
    }
    return nodes.find_by_name("start").unwrap();
}

fn find_path_count_to_end(start: NodeLink, small_cave_visits_left: u32) -> u32 {
    let mut total_paths = 0;
    let mut should_unvisit_at_end = false;
    if start.borrow().name.to_lowercase() == start.borrow().name && !start.borrow().visited {
        start.borrow_mut().visited = true;
        should_unvisit_at_end = true;
    }
    for node in &start.borrow().connected.0 {
        if node.borrow().name == "end" {
            total_paths += 1;
        } else if !node.borrow().visited {
            total_paths += find_path_count_to_end(Rc::clone(&node), small_cave_visits_left);
        } else if small_cave_visits_left > 0 && node.borrow().name != "start" {
            total_paths += find_path_count_to_end(Rc::clone(&node), 0);
        }
    }
    if should_unvisit_at_end {
        start.borrow_mut().visited = false;
    }
    return total_paths;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        // no additional args; print help text
        eprintln!("Usage: {} infile.txt", args[0]);
        return;
    }

    let filename = &args[1];
    let raw_inputs = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let edges = parse_edge_list(raw_inputs.as_str());
    // println!("{}", format_edges(edges));
    println!("{}", find_path_count_to_end(edges, 1));
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiny_input() {
        assert_eq!(find_path_count_to_end(parse_edge_list("start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end"), 1), 36);
    }

    #[test]
    fn test_medium_input() {
        assert_eq!(find_path_count_to_end(parse_edge_list("dc-end
            HN-start
            start-kj
            dc-start
            dc-HN
            LN-dc
            HN-end
            kj-sa
            kj-HN
            kj-dc"), 1), 103);
    }

    #[test]
    fn test_large_input() {
        assert_eq!(find_path_count_to_end(parse_edge_list("fs-end
            he-DX
            fs-he
            start-DX
            pj-DX
            end-zg
            zg-sl
            zg-pj
            pj-he
            RW-he
            fs-DX
            pj-RW
            zg-RW
            start-pj
            he-WI
            zg-he
            pj-fs
            start-RW"), 1), 3509);
    }
}
