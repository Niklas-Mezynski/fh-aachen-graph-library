use super::{WeightedEdge, WithID};

pub type VertexIDType = u32;
pub type EdgeWeight = f64;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub id: VertexIDType,
}

impl WithID for Vertex {
    type IDType = VertexIDType;

    fn get_id(&self) -> VertexIDType {
        self.id
    }
}

#[derive(Debug, Clone)]
pub struct EdgeWithWeight {
    pub weight: EdgeWeight,
}

impl EdgeWithWeight {
    pub fn new(weight: EdgeWeight) -> Self {
        EdgeWithWeight { weight }
    }
}

impl WeightedEdge for EdgeWithWeight {
    type WeightType = EdgeWeight;
    fn get_weight(&self) -> Self::WeightType {
        self.weight
    }
}
