#![allow(dead_code, unused_variables)]
use petgraph::{graph::{NodeIndex, UnGraph}, stable_graph::StableGraph};
use petgraph::Undirected;
use std::ops::Range;
use std::iter::zip;

/// Calculates the matching polynomial of a graph
/// using the fundamental theorem of matching polynomials theorem originally
/// due to Farrell
fn linear_graph(node_count: u32) -> StableGraph<i32, (), Undirected> {
    //let edges_list = vec![];
    let linear_edges_beginnings = Range{start:0, end:node_count-1};
    let linear_edges_ends = Range{start:1, end:node_count};
    let mut edges = zip(linear_edges_beginnings, linear_edges_ends);
    let ungraph = UnGraph::from_edges(&mut edges);
    StableGraph::from(ungraph)
}

fn loop_graph(node_count: u32) -> StableGraph<i32, (), Undirected> {
    //let edges_list = vec![];
    let mut loop_graph = linear_graph(node_count);
    loop_graph.add_edge(NodeIndex::new((node_count - 1) as usize), NodeIndex::new(0), ());
    loop_graph
}

fn fully_connected_graph(node_count: u32) -> StableGraph<i32, (), Undirected> {
    //let rtype = Type::Undirected;
    let fully_connected_graph = UnGraph::<i32, ()>::new_undirected();

    let mut fully_connected_graph = StableGraph::from(fully_connected_graph);
    for i in 0..node_count as i32 {
        fully_connected_graph.add_node(i);
    }
    for i in 0..node_count {
        for j in 0..node_count {
            if i != j {
                fully_connected_graph.add_edge(NodeIndex::new(i as usize), NodeIndex::new(j as usize), ());
            }
        }
    }
    fully_connected_graph
}

//#[pyfunction]
//pub fn calculate_matching_polynomial_from_binary_representation(data: [usize; mem::size_of::<usize>()]) -> Result<Vec<i32>, std::io::Error> {
    //// Produce the graph
    //let graph = Graph::new(data);

    //// now get that polynomial!
    //let poly = _calculate_matching_polynomial_binary(graph);

    //Ok(poly.data().to_vec())
//}




//pub fn _calculate_matching_polynomial_binary(graph: Graph) -> Result<Vec<i32>, std::io::Error> {
    //// now get that polynomial!
    //if graph.edge_count() == 0 { // i.e. we're at the base case.
        //// produce a sequenceof coefficients the same length as the number of vertices
        //let mut coeffics = vec![0; graph.node_count()];
        //coeffics.push(1);
        //return Ok(coeffics)
    //} else {
        //let graph_prime = graph.drop_last_edge();
        //let graph_prime_prime = graph.drop_last_node();

        //let poly_1 = _calculate_matching_polynomial_binary(graph_prime);
        //let poly_2 = _calculate_matching_polynomial_binary(graph_prime_prime);
        //let poly = poly_1 - poly_2;
        //return Ok(poly)
    //}
//} 


//#[pymodule]
//fn match_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    //m.add_function(wrap_pyfunction!(calculate_matching_polynomial, m)?)?;
    //Ok(())
//}
