use std::fmt::Debug;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphError<VId> {
    #[error("Vertex with ID {0} not found")]
    VertexNotFound(VId),

    #[error("Vertex with ID {0} already exists")]
    DuplicateVertex(VId),

    #[error("Edge between vertices {0} and {1} already exists")]
    DuplicateEdge(VId, VId),

    #[error("Invalid file format: {0}")]
    InvalidFormat(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Graph operation failed: {0}")]
    OperationFailed(String),

    #[error("Cannot apply directed operation on undirected graph")]
    DirectedOperationOnUndirectedGraph,

    #[error("Cannot apply undirected operation on directed graph")]
    UndirectedOperationOnDirectedGraph,

    #[error("Algorithm error: {0}")]
    AlgorithmError(String),
}
