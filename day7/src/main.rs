use std::collections::HashMap;

#[derive(Default, Debug)]
struct FileTree<'a> {
    nodes: HashMap<u32, FileTreeNode<'a>>,
}

#[derive(Debug)]
struct FileTreeNode<'a> {
    children: HashMap<&'a str, Vec<FileTreeNodeChildren<'a>>>,
}

#[derive(Debug)]
struct FileTreeNodeChildren<'a> {
    parent_dir: &'a str,
    name: &'a str,
    size: u32,
}

impl<'a> FileTreeNode<'a> {
    fn new() -> Self {
        FileTreeNode {
            children: HashMap::new(),
        }
    }

    fn insert_file(node: &mut Self, file: FileTreeNodeChildren<'a>) {
        if let Some(files) = node.children.get_mut(file.parent_dir) {
            files.push(file)
        };
    }
}

impl FileTree<'_> {
    fn new() -> Self {
        FileTree {
            nodes: HashMap::new(),
        }
    }

    fn insert_dir(tree: &mut Self, depth: u32) {
        tree.nodes.insert(depth, FileTreeNode::new());
    }
}

fn main() {
    println!("Hello, world!");
}
