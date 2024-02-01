use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Node {
    pub elements: Vec<Element>,
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elements = s.chars()
            .map(|c| Element::Token(c))
            .collect::<Vec<_>>();

        Ok(Self { elements })
    }
}

#[derive(Debug, Clone)]
pub enum Element {
    Node(Box<Node>),
    Token(char)
}