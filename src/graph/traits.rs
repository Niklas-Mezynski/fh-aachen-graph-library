use std::fmt::Debug;

pub trait WithID<BaseType, IDType> {
    fn get_id(&self) -> IDType;
}

pub trait GraphInterface<VId, Vertex: WithID<Vertex, VId>, Edge>: Debug {
    // Basic Graph operations
    fn push_vertex(&mut self, vertex: Vertex);
    fn push_edge(&mut self, from: &Vertex, to: &Vertex, edge: Edge);
    fn push_undirected_edge(&mut self, from: &Vertex, to: &Vertex, edge: Edge);

    // Graph queries
    fn get_all_vertices(&self) -> Vec<&Vertex>;
    // fn has_vertex(&self, vertex: &Vertex) -> bool;
    // fn has_edge(&self, from: &Vertex, to: &Vertex) -> bool;
    // fn neighbors(&self, vertex: &Vertex) -> Vec<&Vertex>;
}
