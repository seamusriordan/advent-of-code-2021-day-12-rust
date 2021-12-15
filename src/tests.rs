#[cfg(test)]

mod tests {
    use crate::Graph;

    #[test]
    fn sample_input_has_10_paths() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end".lines();

        let graph = Graph::from_lines(input);

        let paths = graph.get_paths();

        for path in &paths {
            println!("{:?}", path);
        }

        assert_eq!(10, paths.len());
    }
}