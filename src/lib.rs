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

    fn add_child(&self, child: &Rc<Node>) {
        self.child.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = Rc::downgrade(&Rc::new(self.clone()));
    }

    pub fn get_tree(&self) -> String {
        let mut result: Vec<String> = Vec::new();
        let mut level = vec![Rc::new(self.clone())];

        while !level.is_empty() {
            let cur_level: Vec<String> = level.iter().map(|node| node.value.to_string()).collect();
            result.push(cur_level.join(" "));

            level = level
                .iter()
                .flat_map(|node| node.child.borrow().clone())
                .collect()
        }

        result.join("\n")
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            parent: RefCell::new(self.parent.borrow().clone()),
            child: RefCell::new(self.child.borrow().clone()),
        }
    }
}

pub fn gen_tree() -> Rc<Node> {
    let root = Rc::new(Node::new(1));
    let leaf_1 = Rc::new(Node::new(2));
    let leaf_2 = Rc::new(Node::new(3));
    let leaf_3 = Rc::new(Node::new(4));
    let leaf_4 = Rc::new(Node::new(5));
    let leaf_5 = Rc::new(Node::new(6));
    let leaf_6 = Rc::new(Node::new(7));

    leaf_1.add_child(&leaf_3);
    leaf_1.add_child(&leaf_4);
    leaf_2.add_child(&leaf_5);
    leaf_2.add_child(&leaf_6);
    root.add_child(&leaf_1);
    root.add_child(&leaf_2);

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_print_1depth_tree() {
        let tree = Rc::new(Node::new(1));
        assert_eq!(tree.get_tree(), "1");
    }

    #[test]
    fn it_print_2depth_tree() {
        let tree = Rc::new(Node::new(1));
        let leaf_1 = Rc::new(Node::new(2));
        let leaf_2 = Rc::new(Node::new(3));

        tree.add_child(&leaf_1);
        tree.add_child(&leaf_2);
        assert_eq!(tree.get_tree(), "1\n2 3");
    }
}
