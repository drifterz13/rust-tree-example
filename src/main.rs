use rust_tree::{gen_tree, get_tree};

fn main() {
    let tree = gen_tree();
    let mut result = vec![];
    let tree_str = get_tree(&vec![tree], &mut result);
    println!("{tree_str}");
}
