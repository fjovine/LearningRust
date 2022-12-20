mod node;
use node::Node;

fn main() {
    let root = Node::new("1")
      .left(Node::new("1.1")
        .left(Node::new("1.1.1")))
      .right(Node::new("1.2")
        .right(Node::new("1.2.1")));
    root.print(0);
    println!("{}", root.to_string());

}
