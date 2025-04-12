use super::WithID;

pub type VertexIDType = u32;
pub type EdgeWeight = f64;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub id: VertexIDType,
}

impl WithID<VertexIDType> for Vertex {
    fn get_id(&self) -> VertexIDType {
        self.id
    }
}

#[derive(Debug, Clone)]
pub struct WeightedEdge {
    pub weight: EdgeWeight,
}

impl WeightedEdge {
    pub fn new(weight: EdgeWeight) -> Self {
        WeightedEdge { weight }
    }
}
