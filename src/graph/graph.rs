use std::fmt::Debug;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rustc_hash::FxHashSet;

use super::traits::{GraphInterface, WithID};

type VertexIDType = u32;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub id: VertexIDType,
}

impl WithID<Vertex, VertexIDType> for Vertex {
    fn get_id(&self) -> VertexIDType {
        self.id
    }
}

#[derive(Debug)]
pub struct Graph<VId, Vertex: WithID<Vertex, VId>, Edge> {
    backend: dyn GraphInterface<VId, Vertex, Edge>,
}

impl<Edge> Graph<VertexIDType, Vertex, Edge>
where
    Self: Sized,
{
    /// Creates a new graph, from given vertices and edges
    pub fn from(
        n_vertices: VertexIDType,
        vertices: Vec<Vertex>,
        edges: Vec<(Vertex, Vertex, Edge)>,
        directed: bool,
    ) -> Self {
        todo!()
    }

    /// Creates a new graph from a file provided by Prof. Hoever for testing the algorithms.
    ///
    /// Format:
    /// - Erste Zeile: Knotenanzahl
    /// - Folgende Zeilen: Kanten (i->j, Nummerierung: 0 ... Knotenanzahl-1)
    pub fn from_hoever_file(path: &str, edge_builder: fn(remaining: Vec<&str>) -> Edge) -> Self {
        // Read the file line by line
        // Open the file in read-only mode.
        let file = File::open(path).expect("File must exist");

        // Read the file line by line, and return an iterator of the lines.
        let reader = io::BufReader::new(file);

        let mut n_vertices = None;
        let mut vertices: Vec<Vertex> = vec![];
        let mut vertex_ids: FxHashSet<VertexIDType> = FxHashSet::default();
        let mut edges: Vec<(Vertex, Vertex, Edge)> = vec![];

        for (line_number, line) in reader.lines().enumerate() {
            let line = line.unwrap_or_else(|_| panic!("Error reading line {}", line_number));

            match line_number {
                // Parse first line (Number of vertices)
                0 => {
                    n_vertices = Some(
                        line.parse::<VertexIDType>()
                            .expect("First line must be an integer (i.e. the number of vertices)"),
                    )
                }
                // Parse edges
                _ => {
                    let mut parsed_line = line.split("\t");

                    fn get_next_vertex_id(
                        parsed_line: &mut std::str::Split<'_, &str>,
                    ) -> VertexIDType {
                        parsed_line
                            .next()
                            .expect("Each edge must contain at least two values")
                            .parse::<VertexIDType>()
                            .expect("Edges must be represented by integer pairs")
                    }

                    let from = get_next_vertex_id(&mut parsed_line);
                    let to = get_next_vertex_id(&mut parsed_line);

                    let edge = edge_builder(parsed_line.collect::<Vec<&str>>());

                    if !vertex_ids.contains(&from) {
                        vertex_ids.insert(from);
                        vertices.push(Vertex { id: from });
                    }
                    if !vertex_ids.contains(&to) {
                        vertex_ids.insert(to);
                        vertices.push(Vertex { id: to });
                    }

                    edges.push((Vertex { id: from }, Vertex { id: to }, edge));
                }
            }
        }

        Graph::from(
            n_vertices.expect("Must exist at this point"),
            vertices,
            edges,
            false, // TODO: Figure out when to set this
        )
    }
}
