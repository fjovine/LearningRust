// guardare https://dawchihliou.github.io/articles/binary-tree-insertion-in-rust
pub enum NodeRef {
    Some(Box<Node>),
    None
}

pub struct Node {
    pub payload : String,
    pub left : NodeRef,
    pub right : NodeRef,
}

impl Node {
    pub fn new(s : &str) -> Node {
        let result = Node {
            payload : String::from(s),
            left : NodeRef::None,
            right : NodeRef::None,
        };
        
        result
    }

    pub fn set_left(&self, node : &mut Node) {
        self.left = Box::<node>;
    }
}

#[cfg(test)]
mod tests {

}