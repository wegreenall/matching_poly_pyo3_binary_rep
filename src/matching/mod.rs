mod matchings;
mod binary_representation;
mod petgraph;

pub use self::binary_representation::{Graph, calculate_matching_polynomial_from_binary_representation};
pub use self::petgraph::{calculate_matching_polynomial_from_edges, calculate_matching_polynomial_from_adjacency};
