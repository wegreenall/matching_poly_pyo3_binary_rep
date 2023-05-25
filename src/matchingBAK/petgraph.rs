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

pub fn _calculate_matching_polynomial(graph: StableGraph<i32, (), Undirected>) -> Polynomial<u64> {
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
        let poly = poly_1 + poly_2;
        return poly
    }
}

fn get_deck(graph: StableGraph<i32, (), Undirected>) -> Vec<StableGraph<i32, (), Undirected>> {
    let mut deck = Vec::new();
    for node in graph.node_indices() {
        let mut new_graph = graph.clone();
        new_graph.remove_node(node);
        deck.push(new_graph);
    }
    deck
}

pub fn get_matching_polies_stable_graph(graph: StableGraph<i32, (), Undirected>) -> Vec<Vec<u64>> {
    let mut polies = Vec::<Vec<u64>>::new();
    let deck = get_deck(graph.clone());

    // get the matching polynomial of the graph
    let matching_poly = _calculate_matching_polynomial(graph);
    polies.push(matching_poly.data().to_vec());

    // also get the deck
    for subgraph in deck {
        let matching_poly = _calculate_matching_polynomial(subgraph);
        polies.push(matching_poly.data().to_vec());
    }

    polies
}
