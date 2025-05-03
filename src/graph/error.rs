use std::{
    error::Error,
    fmt::{Debug, Display},
};

use thiserror::Error;

use crate::algorithms::mst::union_find::UnionFindError;

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
    AlgorithmError(#[from] Box<dyn Error + 'static>),
}

impl<VId: Debug + Display + 'static> From<UnionFindError<VId>> for GraphError<VId> {
    fn from(value: UnionFindError<VId>) -> Self {
        Self::AlgorithmError(Box::new(value))
    }
}
