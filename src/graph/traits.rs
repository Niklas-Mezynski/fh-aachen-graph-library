use std::fmt::Debug;

pub trait WithID<BaseType, IDType> {
    fn get_id(&self) -> IDType;
}

pub trait GraphInterface<VId, Vertex: WithID<Vertex, VId>, Edge>: Debug {
    // Grundlegende Graphenoperationen
    fn push_vertex(&mut self, vertex: Vertex);
    fn push_edge(&mut self, from: &Vertex, to: &Vertex, edge: Edge);

    // Abfragemethoden
    // fn has_vertex(&self, vertex: &Vertex) -> bool;
    // fn has_edge(&self, from: &Vertex, to: &Vertex) -> bool;
    // fn neighbors(&self, vertex: &Vertex) -> Vec<&Vertex>;
}
