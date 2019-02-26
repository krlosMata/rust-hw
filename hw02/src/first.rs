use std::mem;
// Define structures
/**
 * BST - Binary Tree Search
 */
#[derive(Debug)]
pub struct BST {
    root: Link,
}
/**
 * Link - Defines value of each BST leaf. It could be:
 *  - Empty: leaf has no value
 *  - Node: leag has struct Node type
 */
#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

/**
 * Node - Defines struct for a non-empty leaf os the BST. The struture is ad follows:
 * - value {i32}: value of the leaf
 * - left {Link}: reference to the next leaf placed at left
 * - right {Link}: reference to the next leaf placed at right
 */
#[derive(Debug)]
struct Node {
    value: i32,
    left: Link,
    right: Link,
}

/**
 * Define methods for Node stucture
 */
impl Node {
    fn new(value: i32) -> Box<Node> {
        Box::new(Node {
            value: value,
            left: Link::Empty,
            right: Link::Empty,
        })
    }
}

/**
 * Defining methods for Link Enum type
 */
impl Link {
    fn new(value: i32) -> Self {
        Link::More(Node::new(value))
    }

    fn insert(&mut self, value: i32) -> bool {
        match self {
            &mut Link::Empty => {
                *self = Link::new(value);
                true
            }
            &mut Link::More(ref mut node) => {
                if value < node.value {
                    node.left.insert(value)
                } else if value == node.value {
                    false
                } else {
                    node.right.insert(value)
                }
            }
        }
    }
}

/**
 * Defining methods for BST struct
 */
impl BST {
    pub fn new() -> Self {
        BST { root: Link::Empty }
    }

    pub fn insert(&mut self, elem: i32) -> bool {
      self.root.insert(elem)
    }
}

/**
 * Start unit test section
 */
#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn basics() {
        let mut bst = BST::new();
    }
}