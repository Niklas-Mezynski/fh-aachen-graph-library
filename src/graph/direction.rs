pub trait Direction: 'static + Sized {}

#[derive(Debug)]
pub struct Directed;

#[derive(Debug)]
pub struct Undirected;

impl Direction for Directed {}
impl Direction for Undirected {}
