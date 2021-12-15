use day_12_rust::Graph;

fn main() {
    let input = "YW-end
DK-la
la-XG
end-gy
zq-ci
XG-gz
TF-la
xm-la
gy-gz
ci-start
YW-ci
TF-zq
ci-DK
la-TS
zq-YW
gz-YW
zq-gz
end-gz
ci-TF
DK-zq
gy-YW
start-DK
gz-DK
zq-la
start-TF
".lines();

    let graph = Graph::from_lines(input);

    let paths = graph.get_paths();

    for path in &paths {
        println!("{:?}", path);
    }
    println!("{}", paths.len());
}
