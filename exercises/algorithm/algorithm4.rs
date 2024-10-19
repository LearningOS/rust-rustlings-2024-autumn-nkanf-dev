/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + std::fmt::Debug + Clone,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        self.root = Self::insert_recursive(self.root.take(), value);
    }

    fn insert_recursive(node: Option<Box<TreeNode<T>>>, value: T) -> Option<Box<TreeNode<T>>> {
        match node {
            Some(mut node) => {
                if node.value > value {
                    node.left = Self::insert_recursive(node.left.take(), value);
                } else if node.value < value {
                    node.right = Self::insert_recursive(node.right.take(), value);
                } else {
                    // Value already exists, do not insert
                    return Some(node);
                }
                Some(node)
            }
            None => Some(Box::new(TreeNode::new(value))),
        }
    }

    fn search(&self, value: T) -> bool {
        Self::search_recursive(&self.root, value)
    }

    fn search_recursive(node: &Option<Box<TreeNode<T>>>, value: T) -> bool {
        match node {
            Some(n) => {
                if n.value == value {
                    true
                } else if n.value > value {
                    Self::search_recursive(&n.left, value)
                } else {
                    Self::search_recursive(&n.right, value)
                }
            }
            None => false,
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut bst = BinarySearchTree::new();
        bst.insert(1);
        assert_eq!(bst.root.unwrap().value, 1);
    }

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
