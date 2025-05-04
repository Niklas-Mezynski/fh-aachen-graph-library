use crate::{
    graph::{GraphBase, WithID},
    Graph,
};

impl<Backend> Graph<Backend>
where
    Backend: GraphBase,
    <Backend::Vertex as WithID>::IDType: PartialEq + Copy,
{
    #[allow(clippy::type_complexity)]
    pub fn get_initial_vertex(
        &self,
        start_vertex_id: Option<<Backend::Vertex as WithID>::IDType>,
    ) -> Option<(
        <<Backend as GraphBase>::Vertex as WithID>::IDType,
        impl Iterator<Item = <Backend::Vertex as WithID>::IDType> + use<'_, Backend>,
    )> {
        match start_vertex_id {
            Some(start_vid) => {
                let start_v = self.get_vertex_by_id(start_vid)?.get_id();
                Some((
                    start_v,
                    Box::new(
                        self.get_all_vertices()
                            .map(|v| v.get_id())
                            .filter(move |v| v != &start_v),
                    )
                        as Box<dyn Iterator<Item = <Backend::Vertex as WithID>::IDType> + '_>,
                ))
            }
            None => {
                let mut vertices = self.get_all_vertices().map(|v| v.get_id());
                let start_v = vertices.next()?;

                Some((
                    start_v,
                    Box::new(vertices)
                        as Box<dyn Iterator<Item = <Backend::Vertex as WithID>::IDType> + '_>,
                ))
            }
        }
    }
}
