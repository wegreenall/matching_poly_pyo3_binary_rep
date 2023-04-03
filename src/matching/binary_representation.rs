//#![allow(dead_code, unused_variables)]
use core::fmt;
use std::mem::size_of;
use pyo3::prelude::*;
use polynomial::Polynomial;
use std::mem;
use std::thread;

const MAX_NODES: usize = mem::size_of::<usize>()*8;

/// We represent graphs as a seequence of integers in which each bit represents
/// an edge. The first bit however represents whether that node is contained
/// in the graph; removing a node implies zero-ing this bit.
#[derive(Debug, Clone)]
pub struct Graph {
    data: [usize; size_of::<usize>()*8],
}

impl Graph {
    fn new(data: [usize; size_of::<usize>()*8]) -> Graph {
        Graph {
            data,
        }
    }
    pub fn remove_node(&mut self, node: usize, graph_size : usize) {
        // remove node from adjacency list
        self.data[node] = 0;
        // zero the ith from the left
        //self.data[node] &= !(1 << (graph_size.saturating_sub(node+1))); 
        self.data.iter_mut()
            .for_each(|x| *x &= !(1<<graph_size.saturating_sub(node+1)));
    }

    pub fn remove_edge(&mut self, node1: usize, node2: usize, graph_size: usize) {
        // remove edge from adjacency list
        let shift = graph_size.saturating_sub(node2);
        //println!("before edge removal: {:b}", self.data[node1]);
         
        // zero the ith from the right
        println!("shift: {}", shift);
        println!("graph_size: {}", graph_size);
        self.data[node1] &= (!(1 << (shift-1))) as usize; 
    }

    pub fn edgeless_node_count(&self) -> usize {
        // count the number of nodes in the graph
        // To count the number of nodes in th graph,
        // count the number of the bits in the graph
        // Since, by assumption, the graph is edgeless,
        // we can just count the number of 1s in the graph
        self.data
            .iter()
            .sum::<usize>()
            .count_ones() as usize
    }

    pub fn graph_size(&self) -> usize{
        self.data
            .iter()
            //.enumerate()
            //.map(|(i, x)| x>>i)
            .filter(|x| x> &&(0 as usize)) // i.e. get the ones that are valid
            .count()
    }
    //pub fn edge_count(&self) -> usize {
        //// count the number of edges in the graph
        //self.data
            //.iter()
            //.enumerate()
            //.map(|(i, x)| x>>i)
            //.filter(|x| (x & 1)==1) // i.e. get the ones that are valid
            //.map(|x| x.count_ones()) // count the number of ones in each, and subtract the
                                       //// one that represents the node itself
            //.count()
    //}
    /// checks whether the graph is edgeless, i.e. if each of the elements
    /// is a power of two or 0
    pub fn edgeless(&self) -> bool {
        self.data
            .iter()
            .all(|x| x == &(0 as usize) || x.is_power_of_two())
            //.all(|x| x.is_power_of_two())
    } 

    pub fn get_graph_primes(self) -> (Graph, Graph) {
        let mut new_graph = self.clone();
        let mut new_graph2 = self.clone();

        // get the relevant edge
        let (start_node, end_node, graph_size) = self.get_relevant_edge();

        // G' = G - e
        new_graph.remove_edge(start_node, end_node, graph_size);
        
        // G'' = G - {v, w} where {w, v} are the nodes connected to e
        new_graph2.remove_node(start_node, graph_size);
        new_graph2.remove_node(end_node, graph_size);
        (new_graph, new_graph2)
    }
    /// To step through and calculate the matching polynomial, we use the edge
    /// remove recurrence:
    /// Q(g, x) = Q(G', x) - Q(G'', x)
    ///
    /// G' = G - e
    /// G'' = G - {v, w} where {w, v} are the nodes
    /// at the ends of e
    ///
    /// Thus, we get get the "relevant edge e" which is the first edge in the
    /// first remaining node. Since the nodes are ordered in decreasing order o
    /// f degree, dropping the first edge we find drops the most edges from the
    /// graph, since it the nodes at its ends will be the "most connected" 
    /// nodes.
    fn get_relevant_edge(&self) -> (usize, usize, usize) {
        // since the nodes are ordered in INCREASING order of degree, we can
        // just drop the first edge we find, on the first still-relevant node.
        // starting_node: the index of the first node that still has edges from it

        // if drop_most_connected_edge is true, we delete the first connected edge
        // we find on the first relevant node. Otherwise, we delete the "last"
        // edge for the first relevant node.
        let drop_most_connected_edge: bool = false;

        let starting_node = self.data
            .iter()
            .enumerate()
            .filter(|(_, x)| (x > &&(0 as usize)))
            .filter(|(_, x)| !(x.is_power_of_two()))
            .next()
            .unwrap()
            .0;

        // the first relevant node is a number like: (1, 0, 0, 1, 1, 0, 1). 
        // the next one would be e.g.:               (0, 1, 1, 0, 1, 1, 0) 
        // i.e. 1 on the diagonal.
        // The RELEVANT data, not including the diagonal one to represent its inclusion in the
        // graph, is then the node minus the 1 on the diagonal.
        // Later, we will calculate:
        //              node_data - (1<<node_index)
        let starting_node_data = self.data[starting_node];
        
        // now we have the relevant starting node, we can calculate the edge to drop
        // by finding the LAST bit that is set to 1. First, however, we need to
        // calculate the offset due to non-usize sized grpahs.
                                                                                
        //The first relevant node has some leading zeros up to its relevant diagonal,
        //a 1, and then a set of leading zeros up to the first edge.
        // Comparison point: number of zeros from start of adjacency until the graph information
        // starts.
        let comparison_point = starting_node_data.leading_zeros() as usize - starting_node;
        let graph_size = MAX_NODES.saturating_sub(comparison_point);
        // clean starting node data: the integer corresponding to the node with its first power of two
        // removed
        //  the edge to drop goes between the starting node and the end of the first edge
        let clean_starting_node_data = starting_node_data &!(1<<(graph_size - starting_node - 1));
        let mut end_node: usize = 0;
        if drop_most_connected_edge {
            end_node = clean_starting_node_data.leading_zeros() as usize - comparison_point;
        } else {
            // the edge to drop goes between the starting node and the end of its last edge
            let trailing_zeros = clean_starting_node_data.trailing_zeros() as usize + 1;
            end_node = graph_size.saturating_sub(trailing_zeros);
        }
        let print_stuff: bool = true;
        if print_stuff {
            println!("\n");
            println!("starting_node {}", starting_node);
            println!("starting_node_data {:b}", starting_node_data);
            println!("MAX NODES: {}", MAX_NODES);
            println!("graph_size: {} ", graph_size);
            println!("comparison_point {}", comparison_point);
            println!("clean_starting_node_data {:b}", clean_starting_node_data);
            println!("clean_starting_node_data leading zeros: {}", clean_starting_node_data.leading_zeros());
            println!("end_node {}", end_node);
            println!("\n");
        }
        (starting_node, end_node, graph_size)
    }
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = write!(f, "\n");
        for x in self.data.iter() {
            result = write!(f, "{}\n", x);
        }
        result
    }
}


pub fn get_deck(graph: &Graph) -> Vec<Graph>{
    let mut deck = Vec::<Graph>::new();
    let graph_size = graph.graph_size();
    for i in 0..graph_size {
        //println!("current graph: {}", current_graph);
        let mut current_graph = graph.clone();
        current_graph.remove_node(i, graph_size); 
        deck.push(current_graph.clone());
    }
    deck
}

#[pyfunction]
pub fn calculate_matching_polynomial_from_binary_representation_multithreaded(data: [usize; mem::size_of::<usize>()*8]) -> Result<Vec<Vec<u64>>, std::io::Error> {
    let graph = Graph::new(data);
    let deck = get_deck(&graph);
    let mut polies = Vec::<Vec<u64>>::new();

    let mut thread_handles = Vec::<thread::JoinHandle<Polynomial<u64>>>::new();

    // now get that polynomial!
    let graph_poly = _calculate_matching_polynomial_binary(graph);
    polies.push(graph_poly.data().to_vec());

    for graph in deck {
        // spawn a thread for each graph in the deck
        let handle = thread::spawn(move || {
            _calculate_matching_polynomial_binary(graph)
        });
        thread_handles.push(handle);
        //let graph_poly = _calculate_matching_polynomial_binary(graph);
        //polies.push(graph_poly.data().to_vec());
    }
    for handle in thread_handles {
        let graph_poly = handle.join().unwrap();
        polies.push(graph_poly.data().to_vec());
    }

    Ok(polies)

}

#[pyfunction]
pub fn calculate_matching_polynomial_from_binary_representation(data: [usize; mem::size_of::<usize>()*8]) -> Result<Vec<Vec<u64>>, std::io::Error> {
    let graph = Graph::new(data);
    let deck = get_deck(&graph);
    let mut polies = Vec::<Vec<u64>>::new();

    //let mut thread_handles = Vec::<thread::JoinHandle<Polynomial<u64>>>::new();

    // now get that polynomial!
    let graph_poly = _calculate_matching_polynomial_binary(graph);
    polies.push(graph_poly.data().to_vec());

    for graph in deck {
        // spawn a thread for each graph in the deck
        let poly = _calculate_matching_polynomial_binary(graph);
        polies.push(poly.data().to_vec());
    }

    Ok(polies)
}

pub fn _calculate_matching_polynomial_binary(graph: Graph) -> Polynomial<u64> {
    // the base case for the process is that the graph is edgeless.
    // This means that, of the remaining nodes, each of their integer
    // representations is a power of two.
    if graph.edgeless() { // i.e. we're at the base case.
        // produce a sequence of coefficients the same length as the number of vertices
        //println!("Hit edgeless graph! with {} nodes", graph.edgeless_node_count());
        let mut coeffics = vec![0; graph.edgeless_node_count()];
        coeffics.push(1);
        let poly = Polynomial::new(coeffics);
        //println!("Polynomial: {:?}", poly);
        //println!("graph {:?}", graph.data);
        return poly
    } else {
        // get G' and G''
        // G' = G - an edge
        // G'' = G - the nodes connected to the edge removed to get G'
        //println!("graph {:?}", &graph.data);
        let (graph_prime, graph_prime_prime) = graph.get_graph_primes();
        //println!("graph {:?}", &graph.data);
        //println!("graph_prime {:?}", &graph_prime.data);
        //println!("graph_prime_prime {:?}", &graph_prime_prime.data);

        let poly_1 = _calculate_matching_polynomial_binary(graph_prime);
        let poly_2 = _calculate_matching_polynomial_binary(graph_prime_prime);
        let poly = poly_1 + poly_2;
        return poly
    }
} 

//fn drop_last_nodes(graph: &Graph) -> Graph {
    //// first, get the top node, and its edge.
    ////  the logic will be the same as in the drop_last_edge function
    ////  so we will refactor this out.
    //let new_data = graph.data.clone();
    //let dropped_node = graph
        //.data
        //.iter()
        //.enumerate()
        //.filter(|(_, x)| x > &&(0 as usize))
        //.next();

    //let mut new_graph = Graph::new(new_data);
    //if !dropped_node.is_none() {
        //let dropped_node = dropped_node.unwrap();
        //new_graph.remove_node(dropped_node.0);
    //}
    //new_graph
//}
//fn  get_relevant_edge(graph: &Graph) -> (usize, usize) {
    //// since the nodes are ordered in decreasing order of degree, we can
    //// just drop the first edge we find, on the first still-relevant node.
    //let starting_node = graph.data
        //.iter()
        //.enumerate()
        //.filter(|(_, x)| (x > &&(0 as usize)))
        //.next()
        //.unwrap()
        //.0;

    //// the first reelvant node is a number like: (1, 0, 0, 1, 1, 0, 1). 
    //// the next one would be e.g.:               (0, 1, 1, 0, 1, 1, 0) 
    //// i.e. 1 on the diagonal.
    //// The RELEVANT data then is the node minus the 1 on the diagonal is 
    //// node_data - (1<<node_index)
    //let starting_node_data = graph.data[starting_node];
    ////let starting_node_data = graph.data[starting_node] - (1<<starting_node);
    
    //// now we have the relevant starting node, we can calculate the edge to drop
    //// by finding the first bit that is set to 1
                                                                            
    ////The first relevant node has some leading zeros up to its relevant diagonal,
    ////a 1, and then a set of leading zeros up to the first edge.
    //// Comparison point: number of zeros from start of adjacency until the graph information
    //// starts.
    //let comparison_point = starting_node_data.leading_zeros() as usize - starting_node;
    //// clean starting node data: the integer corresponding to the node with its first power of two
    //// removed
    //let clean_starting_node_data = starting_node_data - (1<<starting_node);

    ////  the edge to drop goes between the starting node and the end of the first edge
    //let end_node = clean_starting_node_data.leading_zeros() as usize - comparison_point;

    //(starting_node, end_node)
//}

//fn drop_last_edge(graph: &Graph) -> Graph {
    //let new_data = graph.data.clone();
    ////let dropped_edge = starting_node_data.leading_zeros() as usize - starting_node_data.leading_zeros() as usize;

    //println!("starting_node: {}", starting_node);
    //println!("dropped edge (i.e. leading zeros): {}", end_node);
    //println!("MAX_NODES: {}", MAX_NODES);

    //// return a graph with the edge dropped
    //let mut new_graph = Graph::new(new_data);
    //new_graph.remove_edge(starting_node, end_node);
    //new_graph
//}
