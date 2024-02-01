use crate::node::Node;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub root_node: Node
}