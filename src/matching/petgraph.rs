#![allow(dead_code, unused_variables)]
use petgraph::{graph::{NodeIndex, UnGraph}, stable_graph::StableGraph};
use petgraph::Undirected;
//use polynomial::Polynomial;
use pyo3::prelude::*;
use polynomial::Polynomial;

fn drop_last_edge(graph: &StableGraph<i32, (), Undirected>) -> StableGraph<i32, (), Undirected> {
    let mut new_graph = graph.clone();
    let edge_count = new_graph.edge_count();
    
    let edge_indices = new_graph.edge_indices();
    let last_edge = edge_indices.last().unwrap();
    new_graph.remove_edge(last_edge);
    new_graph
}

fn drop_last_nodes(graph: &StableGraph<i32, (), Undirected>) -> StableGraph<i32, (), Undirected> {
    // initialise the new graph
    let mut new_graph = graph.clone();

    // get its edge count and use it to get the edges connected to the last node
    let node_count = new_graph.node_count();
    let edge_indices = new_graph.edge_indices();
    let last_edge = edge_indices.last().unwrap();
    let last_nodes = new_graph.edge_endpoints(last_edge);
    //println!("Last nodes: {:?}", last_nodes);
    if !last_nodes.is_none() {
        //println!("Last node zero {:?}", last_nodes.unwrap().0);
        //println!("Last node one {:?}", last_nodes.unwrap().1);
        new_graph.remove_node(last_nodes.unwrap().1);
        new_graph.remove_node(last_nodes.unwrap().0);
    }
    // remove the nodes at the ends of the last edge
    //println!("Last edge weight: {:?}", last);
    //let nodes = last_edge.

    //graph_prime.remove_node(NodeIndex::new(node_count - 2));
    new_graph
}

fn _calculate_matching_polynomial(graph: StableGraph<i32, (), Undirected>) -> Polynomial<i32> {
    //let mut poly = Polynomial::new(vec![0]);
    // get a sequence of zeroes equal to the number of nodes
    if graph.edge_count() == 0 { // i.e. we're at the base case.
        // produce a sequenceof coefficients the same length as the number of vertices
        let mut coeffics = vec![0; graph.node_count()];
        coeffics.push(1);
        //println!("coeffics: {:?}", coeffics);
        let poly = Polynomial::new(coeffics);
        return poly
    } else {
        let graph_prime = drop_last_edge(&graph);
        let graph_prime_prime = drop_last_nodes(&graph);

        let poly_1 = _calculate_matching_polynomial(graph_prime);
        let poly_2 = _calculate_matching_polynomial(graph_prime_prime);
        let poly = poly_1 - poly_2;
        return poly
    }
}

#[pyfunction]
pub fn calculate_matching_polynomial_from_adjacency(input_graph: Vec<Vec<i32>>) -> Result<Vec<i32>, std::io::Error> {
    // Produce the graph
    // First, build the iterator of elements
    // If a node has no edges attached to it, construct a node, else construct the edges
    let node_count = input_graph.len();
    let mut graph = UnGraph::<i32, ()>::with_capacity(node_count, node_count);
    for (i, node) in input_graph.iter().enumerate(){
        // add the nodes and their edges to the graph
        // if the node has no edges, add it as a node
        if node.iter().sum::<i32>() == 0 {
            graph.add_node(1 as i32);
        } else {
            // otherwise, add the edges
            for edge in node {
                graph.add_edge(NodeIndex::new(i), NodeIndex::new(*edge as usize), ());
            }
        }

    }

    let graph = StableGraph::<i32, (), Undirected>::from(graph);

    //// now get that polynomial!
    let poly = _calculate_matching_polynomial(graph);

    Ok(poly.data().to_vec())
} 

#[pyfunction]
pub fn calculate_matching_polynomial_from_edges(graph: Vec<(u32, u32)>) -> Result<Vec<i32>, std::io::Error> {
    // Produce the graph
    let graph = UnGraph::<i32, ()>::from_edges(&graph);
    let graph = StableGraph::<i32, (), Undirected>::from(graph);

    // now get that polynomial!
    let poly = _calculate_matching_polynomial(graph);

    Ok(poly.data().to_vec())
} 
