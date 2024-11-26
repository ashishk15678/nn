// Module: test
mod tokenizer;
#[allow(unused_imports)]
#[allow(non_snake_case)]
#[cfg(test)]
use crate::node::Node;

#[test]
fn testing_test() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn Node_new() {
    let node = Node::new(1.0, 0.5);
    assert_eq!(node.data, 1.0);
    assert_eq!(node.weight, 0.5);
    assert_eq!(node.active, false);
    assert_eq!(node.activation, None);
}
