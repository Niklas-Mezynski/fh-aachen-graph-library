use std::{
    fmt::{Debug, Display},
    fs,
    str::FromStr,
};

use num_traits::{FromPrimitive, ToPrimitive};

use crate::{graph::traits::GraphBase, GraphError};

use super::{Graph, Vertex, VertexIDType, WithID};

impl<Backend> Graph<Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType:
        FromStr + PartialEq + PartialOrd + Copy + Debug + FromPrimitive + ToPrimitive + Display,
{
    /// Creates a new graph from a file provided by Prof. Hoever for testing the algorithms.
    ///
    /// - The first line contains the number of vertices (n)
    /// - The next next n lines contain the vertex data and will call `vertex_builder`
    /// - The remaining lines look like this: `from_vertex_id` -> `to_vertex_id` -> `edge data...`
    pub fn from_hoever_file_with_special_vertices(
        path: &str,
        vertex_builder: fn(index: usize, str_parts: Vec<&str>) -> Backend::Vertex,
        edge_builder: fn(str_parts: Vec<&str>) -> Backend::Edge,
    ) -> Result<Self, GraphError<<Backend::Vertex as WithID>::IDType>> {
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
            .parse::<<Backend::Vertex as WithID>::IDType>()
            .map_err(|_e| {
                GraphError::ParseError("Cannot parse number of vertices (1st line)".to_string())
            })?;

        let n_vertices_usize = n_vertices.to_usize().unwrap();

        if n_vertices_usize == 0 {
            return Err(GraphError::InvalidFormat(
                "Number of vertices must be greater than 0".to_string(),
            ));
        }

        let vertices = line_iter
            .clone()
            .enumerate()
            .map_while(|(i, line)| {
                if i >= n_vertices_usize {
                    return None;
                }

                // Parse the
                let parsed_line = line.split('\t');
                let vertex = vertex_builder(i, parsed_line.collect::<Vec<&str>>());
                Some(vertex)
            })
            .collect::<Vec<_>>();

        let edges = line_iter
            .skip(n_vertices_usize)
            .map(|line| {
                let mut parsed_line = line.split('\t');

                let from = parsed_line
                    .next()
                    .ok_or_else(|| {
                        GraphError::<<Backend::Vertex as WithID>::IDType>::InvalidFormat(
                            "Missing 'from' vertex id in edge definition".to_string(),
                        )
                    })?
                    .parse::<<Backend::Vertex as WithID>::IDType>()
                    .map_err(|_e| {
                        GraphError::ParseError("Cannot parse \"from\" vertex".to_string())
                    })?;

                let to = parsed_line
                    .next()
                    .ok_or_else(|| {
                        GraphError::InvalidFormat(
                            "Missing 'to' vertex id in edge definition".to_string(),
                        )
                    })?
                    .parse::<<Backend::Vertex as WithID>::IDType>()
                    .map_err(|_e| {
                        GraphError::ParseError("Cannot parse \"to\" vertex".to_string())
                    })?;

                // Check if vertex IDs are within valid range
                if from >= n_vertices || to >= n_vertices {
                    return Err(GraphError::InvalidFormat(format!(
                        "Vertex ID out of range: expected 0-{}, got {} or {}",
                        n_vertices_usize - 1,
                        from,
                        to
                    )));
                }

                let edge = edge_builder(parsed_line.collect::<Vec<&str>>());

                Ok((from, to, edge))
            })
            .collect::<Result<Vec<_>, GraphError<<Backend::Vertex as WithID>::IDType>>>()?;

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
    /// - The first line contains the number of vertices (n)
    /// - The remaining lines look like this: `from_vertex_id` -> `to_vertex_id` -> `edge data...`
    pub fn from_hoever_file(
        path: &str,
        vertex_builder: fn(id: <Backend::Vertex as WithID>::IDType) -> Backend::Vertex,
        edge_builder: fn(remaining: Vec<&str>) -> Backend::Edge,
    ) -> Result<Self, GraphError<<Backend::Vertex as WithID>::IDType>> {
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
            .parse::<<Backend::Vertex as WithID>::IDType>()
            .map_err(|_e| {
                GraphError::ParseError("Cannot parse number of vertices (1st line)".to_string())
            })?;

        if n_vertices.to_usize().unwrap() == 0 {
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
                        GraphError::<<Backend::Vertex as WithID>::IDType>::InvalidFormat(
                            "Missing 'from' vertex id in edge definition".to_string(),
                        )
                    })?
                    .parse::<<Backend::Vertex as WithID>::IDType>()
                    .map_err(|_e| {
                        GraphError::ParseError("Cannot parse \"from\" vertex".to_string())
                    })?;

                let to = parsed_line
                    .next()
                    .ok_or_else(|| {
                        GraphError::InvalidFormat(
                            "Missing 'to' vertex id in edge definition".to_string(),
                        )
                    })?
                    .parse::<<Backend::Vertex as WithID>::IDType>()
                    .map_err(|_e| {
                        GraphError::ParseError("Cannot parse \"to\" vertex".to_string())
                    })?;

                // Check if vertex IDs are within valid range
                if from >= n_vertices || to >= n_vertices {
                    return Err(GraphError::InvalidFormat(format!(
                        "Vertex ID out of range: expected 0-{}, got {} or {}",
                        n_vertices.to_usize().unwrap() - 1,
                        from,
                        to
                    )));
                }

                let edge = edge_builder(parsed_line.collect::<Vec<&str>>());

                Ok((from, to, edge))
            })
            .collect::<Result<Vec<_>, GraphError<<Backend::Vertex as WithID>::IDType>>>()?;

        // We create a vertex each for the number of vertices in line 1 (starting at 0)
        let vertices: Vec<_> = (0..n_vertices.to_usize().unwrap())
            .map(|i| vertex_builder(<Backend::Vertex as WithID>::IDType::from_usize(i).unwrap()))
            .collect();

        if edges.is_empty() {
            return Err(GraphError::InvalidFormat(
                "No edges found in file".to_string(),
            ));
        }

        Self::from_vertices_and_edges(vertices, edges)
    }
}

impl<Backend> Graph<Backend>
where
    // Vertex: Debug,
    // Edge: Debug,
    Backend: GraphBase<Vertex = Vertex>,
{
    /// Creates a new graph from a file provided by Prof. Hoever for testing the algorithms.
    pub fn from_hoever_file_with_edges(
        path: &str,
        edge_builder: fn(remaining: Vec<&str>) -> Backend::Edge,
    ) -> Result<Self, GraphError<VertexIDType>> {
        Self::from_hoever_file(path, |id| Vertex { id }, edge_builder)
    }
}

impl<Backend> Graph<Backend>
where
    Vertex: Debug,
    Backend: GraphBase<Vertex = Vertex, Edge = ()>,
{
    /// Creates a new graph from a file provided by Prof. Hoever for testing the algorithms.
    pub fn from_hoever_file_default(path: &str) -> Result<Self, GraphError<VertexIDType>> {
        Self::from_hoever_file(path, |id| Vertex { id }, |_| ())
    }
}
