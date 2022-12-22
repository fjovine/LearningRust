use crate::string_reader;

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

    pub fn left(&mut self, node : Node) {
        self.left = Some(Box::new(node));
    }

    pub fn right(&mut self, node : Node) {
        self.right = Some(Box::new(node));
    }

    fn decode(reader : &mut string_reader::StringReader) -> Node {
        reader.accept('"');
        let s : &str = &reader.get_next_quoted_string()[..];
        let mut result = Node::new(s);
        reader.accept(',');
        match reader.next() {
            Some(c) => 
                if c ==  '(' {
                   result.left(Node::decode(reader));
                } else if c!='0' {
                    panic!("Invalid char");
                },
            None => panic!("String finished too early"),
        }
        reader.accept(',');
        match reader.next() {
            Some(c) => 
                if c ==  '(' {
                   result.right(Node::decode(reader));
                } else if c!='0' {
                    panic!("Invalid char");
                },
            None => panic!("String finished too early"),
        }
        reader.accept(')');
        result
    }    

    pub fn create_from(s:&str) -> Self {
        let mut reader = string_reader::StringReader::new(s);
        println!("1");
        reader.accept('(');
        Node::decode(&mut reader)
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

#[cfg(test)]
mod test {
    fn decode_works_well_check(model :&str, expected:&str)
    {
        let root = super::Node::create_from(model);
        let found = &root.to_string();
        println!("model {} expeced {} found {}", model, expected, found)
        assert!(expected.eq(found));
    }

    #[test]
    fn decode_works_well() {
        decode_works_well_check("(\"AA\", 0  ,    0)","(\"AA\",0,0");
    }
}