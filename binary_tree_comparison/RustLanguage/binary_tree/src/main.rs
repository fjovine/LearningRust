mod node;
mod string_reader;
use node::Node;

fn main() {
/*
    let root = Node::new("1")
      .left(Node::new("1.1")
        .left(Node::new("1.1.1")))
      .right(Node::new("1.2")
        .right(Node::new("1.2.1")));
    root.print(0);
    println!("{}", root.to_string());
*/
  let root = Node::create_from("(\"A\",(\"B\",0,0),(\"C\",(\"D\",0,0),(\"E\",(\"F\",0,0),(\"G\",0,0)))))");
  root.print(0);

}
