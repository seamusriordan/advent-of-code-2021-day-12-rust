use std::str::Lines;

mod tests;

#[derive(Clone)]
pub struct Node {
    value: &'static str,
    nodes: Vec<&'static str>,
}

impl Node {
    pub fn new(value: &'static str) -> Node {
        return Node {
            value,
            nodes: vec![],
        };
    }

    fn from_lines(lines: Lines<'static>, token: &'static str) -> Node {
        let mut node = Node::new(token);

        for line in lines.clone() {
            let mut tokens = line.split("-");
            let head = tokens.next().unwrap();
            let tail = tokens.next().unwrap();

            if head == node.value {
                node.nodes.push(tail)
            }

            if tail == node.value {
                node.nodes.push(head)
            }
        }
        node
    }
}

pub struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn from_lines(lines: Lines<'static>) -> Graph {
        let mut tokens: Vec<&str> = vec![];

        for line in lines.clone() {
            let mut line_tokens = line.split("-");

            while let Some(token) = line_tokens.next() {
                tokens.push(token)
            }
        }

        tokens.dedup_by(|t1, t2| *t1 == *t2);


        let mut graph = Graph {
            nodes: vec![]
        };
        for token in tokens {
            graph.nodes.push(Node::from_lines(lines.clone(), token))
        }

        graph
    }

    pub fn get_paths(&self) -> Vec<Vec<&str>> {
        self.build_path("start", vec![])
    }

    fn build_path<'a>(&self, current_node_value: &'a str, current_path: Vec<&'a str>) -> Vec<Vec<&'a str>> {
        let mut paths = vec![];
        let mut next_path = current_path.clone();
        next_path.push(current_node_value);

        if current_node_value == "end" {
            paths.push(next_path)
        } else {
            let current_node = self.nodes.iter().find(|x| x.value == current_node_value).unwrap();

            for node_value in &current_node.nodes {
                if !current_path.contains(&node_value) || node_value.to_ascii_uppercase() == *node_value {
                    let mut node_paths = self.build_path(node_value, next_path.clone());

                    paths.append(&mut node_paths);
                }
            }
        }

        paths
    }
}