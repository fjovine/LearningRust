// guardare https://dawchihliou.github.io/articles/binary-tree-insertion-in-rust
pub struct Node {
    pub payload : String,
    left : Option<Box<Node>>,
    right : Option<Box<Node>>,
}

impl Node {
    pub fn new(s : &str) -> Self {
        let result = Node {
            payload : String::from(s),
            left : None,
            right : None,
        };
        
        result
    }

    pub fn create_from(s:&str) -> Self {
        Node::new(s)
    }

    pub fn left(mut self, node : Node) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node : Node) -> Self {
        self.right = Some(Box::new(node));
        self
    }

    pub fn print (& self, left_indent : usize) {
        let indentation = " ".repeat(left_indent);

        println!("{}{}", indentation, self.payload);
        match &self.left {
            Some(p) => p.print(left_indent + 2),
            None => println!("{}  0", indentation),
        }
        match &self.right {
            Some(p) => p.print(left_indent + 2),
            None => println!("{}  0", indentation),
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::from("(\"");
        result += &self.payload;
        result += "\",";
        match &self.left {
            Some(p) => result += &p.to_string(),
            None => result += "0",
        }
        result += ",";
        match &self.right {
            Some(p) => result += &p.to_string(),
            None => result += "0",
        }

        result += ")";
        result
    }
}