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
    ParseError(ParsingError),

    #[error("Graph operation failed: {0}")]
    OperationFailed(String),
}

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Failed to parse integer: {0}")]
    Int(#[from] std::num::ParseIntError),
    #[error("Failed to parse float: {0}")]
    Float(#[from] std::num::ParseFloatError),
}
