pub trait Direction: 'static + Sized {}

#[derive(Debug, Clone)]
pub struct Directed;

#[derive(Debug, Clone)]
pub struct Undirected;

impl Direction for Directed {}
impl Direction for Undirected {}
