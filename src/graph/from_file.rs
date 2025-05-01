use std::fs;

use crate::{graph::traits::GraphBase, GraphError};

use super::{error::ParsingError, Graph, Vertex, VertexIDType};

impl<Backend> Graph<Backend> {
    /// Creates a new graph from a file provided by Prof. Hoever for testing the algorithms.
    ///
    /// Format:
    /// - Erste Zeile: Knotenanzahl
    /// - Folgende Zeilen: Kanten (i->j, Nummerierung: 0 ... Knotenanzahl-1)
    pub fn from_hoever_file_with_weights<Edge>(
        path: &str,
        edge_builder: fn(remaining: Vec<&str>) -> Edge,
    ) -> Result<Self, GraphError<VertexIDType>>
    where
        Backend: GraphBase<Vertex, Edge>,
    {
        // Open the file in read-only mode.
        let file_contents = fs::read_to_string(path).map_err(GraphError::IoError)?;
        let mut line_iter = file_contents.lines();

        // Parse first line (number of vertices)
        let n_vertices = line_iter
            .next()
            .ok_or_else(|| {
                GraphError::InvalidFormat(
                    "The file must contain at least one line with the number of edges".to_string(),
                )
            })?
            .parse::<VertexIDType>()
            .map_err(|e| GraphError::ParseError(ParsingError::Int(e)))?;

        if n_vertices == 0 {
            return Err(GraphError::InvalidFormat(
                "Number of vertices must be greater than 0".to_string(),
            ));
        }

        let edges = line_iter
            .map(|line| {
                let mut parsed_line = line.split('\t');

                let from = parsed_line
                    .next()
                    .ok_or_else(|| {
                        GraphError::<VertexIDType>::InvalidFormat(
                            "Missing 'from' vertex id in edge definition".to_string(),
                        )
                    })?
                    .parse::<VertexIDType>()
                    .map_err(|e| GraphError::ParseError(ParsingError::Int(e)))?;

                let to = parsed_line
                    .next()
                    .ok_or_else(|| {
                        GraphError::InvalidFormat(
                            "Missing 'to' vertex id in edge definition".to_string(),
                        )
                    })?
                    .parse::<VertexIDType>()
                    .map_err(|e| GraphError::ParseError(ParsingError::Int(e)))?;

                // Check if vertex IDs are within valid range
                if from >= n_vertices || to >= n_vertices {
                    return Err(GraphError::InvalidFormat(format!(
                        "Vertex ID out of range: expected 0-{}, got {} or {}",
                        n_vertices - 1,
                        from,
                        to
                    )));
                }

                let edge = edge_builder(parsed_line.collect::<Vec<&str>>());

                Ok((from, to, edge))
            })
            .collect::<Result<Vec<_>, GraphError<VertexIDType>>>()?;

        // We create a vertex each for the number of vertices in line 1 (starting at 0)
        let vertices: Vec<Vertex> = (0..n_vertices).map(|vid| Vertex { id: vid }).collect();

        if edges.is_empty() {
            return Err(GraphError::InvalidFormat(
                "No edges found in file".to_string(),
            ));
        }

        Self::from_vertices_and_edges(vertices, edges)
    }

    /// Creates a new graph from a file provided by Prof. Hoever for testing the algorithms.
    ///
    /// Format:
    /// - Erste Zeile: Knotenanzahl
    /// - Folgende Zeilen: Kanten (i->j, Nummerierung: 0 ... Knotenanzahl-1)
    pub fn from_hoever_file(path: &str) -> Result<Self, GraphError<VertexIDType>>
    where
        Backend: GraphBase<Vertex, ()>,
    {
        Self::from_hoever_file_with_weights(path, |_| ())
    }
}
