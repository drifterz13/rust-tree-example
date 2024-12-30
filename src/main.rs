use rust_tree::gen_tree;

fn main() {
    let tree = gen_tree();
    let tree_str = tree.get_tree();
    println!("{tree_str}");
}
