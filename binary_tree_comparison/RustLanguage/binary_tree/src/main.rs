mod node;

fn main() {
    let mut root = node::Node::new("1");
    root.set_left(node::Node::new("1.1"));
/*    root.left.left = node::Node::new("1.1.1");
    root.right = node::Node::new("1.2");
    root.right.left = node::Node::new("1.2.1");
    */
}
