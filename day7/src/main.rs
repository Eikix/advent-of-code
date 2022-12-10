use std::collections::{HashMap, HashSet};

#[derive(Default, Debug, PartialEq, Clone)]
struct FileTree<'a> {
    nodes: HashMap<u32, FileTreeNode<'a>>,
}

#[derive(Debug, PartialEq, Default, Clone)]
struct FileTreeNode<'a> {
    children: HashMap<&'a str, HashSet<FileTreeNodeChild<'a>>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
struct FileTreeNodeChild<'a> {
    parent_dirs: Vec<&'a str>,
    _name: &'a str,
    size: u32,
}

impl<'a> FileTreeNode<'a> {
    fn new() -> Self {
        FileTreeNode {
            children: HashMap::new(),
        }
    }

    fn insert_file(node: &mut Self, file: FileTreeNodeChild<'a>) {
        if let Some(parent_dir) = file.parent_dirs.last() {
            if let Some(files) = node.children.get_mut(parent_dir) {
                files.insert(file);
            } else {
                let mut files = HashSet::new();
                files.insert(file.clone());
                node.children.insert(parent_dir, files);
            }
        }
    }
}

impl FileTree<'_> {
    pub fn new() -> Self {
        FileTree {
            nodes: HashMap::new(),
        }
    }

    fn insert_depth(tree: &mut Self, depth: u32) {
        if tree.nodes.get(&depth).is_none() {
            tree.nodes.insert(depth, FileTreeNode::new());
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines();
    let tree = construct_tree(lines.collect());
    let count: u32 = part_one(tree);

    println!("{:#?}", count);
}

fn construct_tree(lines: Vec<&str>) -> FileTree {
    let mut tree = FileTree::new();
    let mut current_depth = 0;
    let mut current_dirs: Vec<&str> = vec!["/"];

    lines.into_iter().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().into_iter().collect();
        if let Some(command) = parts.first() {
            if *command == "$" {
                if let Some(&"cd") = parts.get(1) {
                    match parts.get(2) {
                        Some(&"/") => {
                            current_depth = 0;
                            current_dirs.clear();
                            current_dirs.push("/");
                        }
                        Some(&"..") => {
                            if current_depth > 0 {
                                current_depth -= 1;
                                current_dirs.pop();
                                if current_dirs.is_empty() {
                                    current_dirs.push("/");
                                }
                            }
                        }
                        Some(dirname) => {
                            current_depth += 1;
                            current_dirs.push(dirname);
                        }
                        _ => {}
                    }
                }
            } else {
                match parts.first() {
                    Some(&"dir") => {}
                    Some(size) => {
                        FileTree::insert_depth(&mut tree, current_depth);
                        if let Some(filename) = parts.get(1) {
                            let file = FileTreeNodeChild {
                                parent_dirs: current_dirs.clone(),
                                _name: filename,
                                size: size.parse().unwrap(),
                            };
                            if let Some(node) = tree.nodes.get_mut(&current_depth) {
                                FileTreeNode::insert_file(node, file);
                            }
                        };
                    }
                    _ => {}
                }
            }
        }
    });

    tree
}

fn compute_dir_sizes(tree: FileTree) -> HashMap<&str, u32> {
    let mut hash_tree_count: HashMap<&str, u32> = HashMap::new();

    tree.nodes.into_values().for_each(|node| {
        node.children.into_values().for_each(|files| {
            files.iter().for_each(|file| {
                file.parent_dirs.clone().into_iter().for_each(|parent_dir| {
                    if let Some(count) = hash_tree_count.get_mut(&parent_dir) {
                        *count += file.size;
                    } else {
                        hash_tree_count.insert(parent_dir, file.size);
                    }
                })
            })
        })
    });

    hash_tree_count
}

fn part_one(tree: FileTree) -> u32 {
    let hash_tree_count = compute_dir_sizes(tree);

    println!("{:#?}", hash_tree_count);

    let count: u32 = hash_tree_count
        .into_values()
        .filter(|size| *size <= 100_000)
        .sum();

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_file_tree() {
        let mut tree = FileTree::new();

        FileTree::insert_depth(&mut tree, 1);

        if let Some(node) = tree.nodes.get_mut(&1) {
            let file_one = FileTreeNodeChild {
                parent_dirs: vec!["/", "dir1"],
                _name: "file1.txt",
                size: 100,
            };

            let file_two = FileTreeNodeChild {
                parent_dirs: vec!["/", "dir1"],
                _name: "file2.txt",
                size: 200,
            };

            FileTreeNode::insert_file(node, file_one.clone());
            FileTreeNode::insert_file(node, file_two.clone());
        };

        let test_file_size: u32 = compute_dir_sizes(tree.clone()).into_values().sum();
        assert_eq!(test_file_size, 600);

        FileTree::insert_depth(&mut tree, 2);

        if let Some(node) = tree.nodes.get_mut(&2) {
            let file_one = FileTreeNodeChild {
                parent_dirs: vec!["/", "dir1", "dir2"],
                _name: "file1.txt",
                size: 300,
            };

            let file_two = FileTreeNodeChild {
                parent_dirs: vec!["/", "dir1", "dir2"],
                _name: "file2.txt",
                size: 500,
            };

            FileTreeNode::insert_file(node, file_one.clone());
            FileTreeNode::insert_file(node, file_two.clone());
        };

        let test_file_size: u32 = compute_dir_sizes(tree.clone()).into_values().sum();
        assert_eq!(test_file_size, 800 + (300 + 800) + 300 + 800);
    }

    #[test]
    fn integration_test() {
        let input_str = "$ cd /\n\
                        $ ls\n\
                        dir a\n\
                        14848514 b.txt\n\
                        8504156 c.dat\n\
                        dir d\n\
                        $ cd a\n\
                        $ ls\n\
                        dir e\n\
                        29116 f\n\
                        2557 g\n\
                        62596 h.lst\n\
                        $ cd e\n\
                        $ ls\n\
                        584 i\n\
                        $ cd ..\n\
                        $ cd ..\n\
                        $ cd d\n\
                        $ ls\n\
                        4060174 j\n\
                        8033020 d.log\n\
                        5626152 d.ext\n\
                        7214296 k\n";
        let lines = input_str.lines();
        let tree = construct_tree(lines.collect());
        let count: u32 = part_one(tree);
        assert_eq!(count, 95437);
    }

    #[test]
    fn test_input() {
        let input_str = include_str!("input.txt");
        let lines = input_str.lines();
        assert_eq!(lines.count(), 942);
    }
}
