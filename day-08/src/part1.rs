use core::panic;
use std::fmt;

#[derive(Debug, Default)]
struct ArenaTree {
    arena: Vec<Node>,
}

struct Node {
    idx: usize,
    name: String,
    left: Option<usize>,
    right: Option<usize>,
}

impl Node {
    fn new(idx: usize, name: &str) -> Self {
        Self {
            idx,
            name: name.to_string(),
            left: None,
            right: None,
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.idx)
    }
}

impl ArenaTree {
    fn node(&mut self, name: &str) -> usize {
        for node in &self.arena {
            if node.name == name {
                return node.idx;
            }
        }
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, name));
        idx
    }
}
pub fn process(input: &str) -> String {
    let instructions: Vec<_> = input.lines().next().unwrap().chars().collect();

    let mut tree: ArenaTree = ArenaTree::default();
    for line in input.lines().skip(2) {
        let name = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        let node = tree.node(name);
        tree.arena[node].right = Some(tree.node(right));
        tree.arena[node].left = Some(tree.node(left));
    }

    let mut node = tree.node("AAA");
    let target_node = tree.node("ZZZ");
    let mut step_count = 0;
    let mut next_instruction = instructions.iter().cycle();
    while node != target_node {
        step_count += 1;
        let left = tree.arena[node].left.unwrap();
        let right = tree.arena[node].right.unwrap();
        match next_instruction.next().unwrap() {
            'L' => node = left,
            'R' => node = right,
            _ => panic!("unexpected character in instructions"),
        }
    }

    step_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "RL\n\
                     \n\
                     AAA = (BBB, CCC)\n\
                     BBB = (DDD, EEE)\n\
                     CCC = (ZZZ, GGG)\n\
                     DDD = (DDD, DDD)\n\
                     EEE = (EEE, EEE)\n\
                     GGG = (GGG, GGG)\n\
                     ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", process(input))
    }
}
