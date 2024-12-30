use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    child: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value,
            parent: RefCell::new(Weak::new()),
            child: RefCell::new(vec![]),
        }
    }
}

fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
    parent.child.borrow_mut().push(Rc::clone(child));
    *child.parent.borrow_mut() = Rc::downgrade(parent);
}

pub fn get_tree(nodes: &Vec<Rc<Node>>, result: &mut Vec<String>) -> String {
    if nodes.len() == 0 {
        return result.join("\n");
    }

    let cur_level: Vec<String> = nodes.iter().map(|node| node.value.to_string()).collect();
    let cur_level = cur_level.join(" ");

    let next_level_nodes: Vec<Rc<Node>> = nodes
        .iter()
        .map(|node| node.child.take())
        .flatten()
        .collect();

    result.push(cur_level);

    return get_tree(&next_level_nodes, result);
}

pub fn gen_tree() -> Rc<Node> {
    let root = Rc::new(Node::new(1));
    let leaf_1 = Rc::new(Node::new(2));
    let leaf_2 = Rc::new(Node::new(3));
    let leaf_3 = Rc::new(Node::new(4));
    let leaf_4 = Rc::new(Node::new(5));
    let leaf_5 = Rc::new(Node::new(6));
    let leaf_6 = Rc::new(Node::new(7));

    add_child(&leaf_1, &leaf_3);
    add_child(&leaf_1, &leaf_4);
    add_child(&leaf_2, &leaf_5);
    add_child(&leaf_2, &leaf_6);

    add_child(&root, &leaf_1);
    add_child(&root, &leaf_2);

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_print_1depth_tree() {
        let root = Rc::new(Node::new(1));
        assert_eq!(get_tree(&vec![root], &mut vec![]), "1");
    }

    #[test]
    fn it_print_2depth_tree() {
        let root = Rc::new(Node::new(1));
        let leaf_1 = Rc::new(Node::new(2));
        let leaf_2 = Rc::new(Node::new(3));

        add_child(&root, &leaf_1);
        add_child(&root, &leaf_2);

        let tree_str = get_tree(&vec![root], &mut vec![]);
        assert_eq!(tree_str, "1\n2 3");
    }
}
