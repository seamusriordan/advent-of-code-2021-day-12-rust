#[cfg(test)]

mod tests {
    use crate::Graph;

//     #[test]
//     fn first_sample_input_has_10_paths() {
//         let input = "start-A
// start-b
// A-c
// A-b
// b-d
// A-end
// b-end".lines();
//
//         let graph = Graph::from_lines(input);
//
//         let paths = graph.get_paths();
//
//         for path in &paths {
//             println!("{:?}", path);
//         }
//
//         assert_eq!(10, paths.len());
//     }

    #[test]
    fn second_sample_input_has_19_paths() {
        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc".lines();

        let graph = Graph::from_lines(input);

        let paths = graph.get_paths();

        for path in paths.clone() {
            println!("{:?}", path);

        }
        assert_eq!(103, paths.len());
    }
    #[test]
    fn third_sample_input_has_226_paths() {
        let input = "fs-end
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
start-RW".lines();

        let graph = Graph::from_lines(input);

        let paths = graph.get_paths();


        assert_eq!(3509, paths.len());
    }
}