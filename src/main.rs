use petgraph::algo::bellman_ford;
use petgraph::Graph;
use petgraph::dot::Dot;

fn main() {
    let mut graph = Graph::<&str, f64>::new();

    let eth = "eth";
    let usdc = "usdc";
    let dai = "dai";



    let a = graph.add_node(&eth);
    let b = graph.add_node(&usdc);
    let c = graph.add_node(&dai);
    graph.add_edge(a, b, -5.0);
    graph.add_edge(b, c, 3.0);
    graph.add_edge(a, c, 7.0);

    let path = bellman_ford(&graph, a);
    assert!(path.is_ok());
    let path = path.unwrap();
    println!("Shortest distances from node a: {:?}", path.distances);
    println!("Predecessors from node a: {:?}", path.predecessors);
    println!("{:?}", Dot::new(&graph));
}
