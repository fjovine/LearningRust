enum NodeRef {
    Some(&mut Node),
    None,
}

struct Node {
    payload : String,
    left : NodeRef,
    right : NodeRef,
}