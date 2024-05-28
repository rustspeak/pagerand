
use pagerand::{Arena, Local};
use petgraph::Graph;
fn main() {
    let arena = Arena::new();

    let mut vec = Vec::new();

    for _ in 0..1000 {
        let s = arena.alloc(Local("Hello, world!"));
        vec.push(s);
    }

    for s in vec {
        arena.free(s);
    }


 let mut graph = Graph::<(), ()>::new();

    let alice = graph.add_node();
    let bob = graph.add_node();
    let carol = graph.add_node();

    graph.add_edge(alice, bob, ());
    graph.add_edge(bob, carol, ());
    graph.add_edge(carol, alice, ());

    for node in graph.nodes() {
        println!("Node: {:?}", node);
    }

    for edge in graph.edges() {
        println!("Edge: {:?} -> {:?}", edge.source(), edge.target());
    }

}
