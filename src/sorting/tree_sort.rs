use std::cell::RefCell;
use std::rc::Rc;

// Define the structure for the BST Node
#[derive(Debug)]
struct Node {
    key: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

pub struct TreeSort;

// Function to create a new BST Node
// This function returns a reference-counted pointer to a newly created Node wrapped in a RefCell
fn new_node(key: i32) -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(Node {
        key,
        left: None,
        right: None,
    }))
}

// Function to insert a new Node with the given key in the BST
// Recursively finds the correct position for the new key and inserts it
// If the tree is empty, it creates and returns a new Node
fn insert(node: &Option<Rc<RefCell<Node>>>, key: i32) -> Option<Rc<RefCell<Node>>> {
    match node {
        Some(current) => {
            let mut current_borrow = current.borrow_mut();
            match key.cmp(&current_borrow.key) {
                // If the key is less, insert in the left subtree
                std::cmp::Ordering::Less => {
                    current_borrow.left = insert(&current_borrow.left, key);
                }
                // If the key is greater, insert in the right subtree
                std::cmp::Ordering::Greater => {
                    current_borrow.right = insert(&current_borrow.right, key);
                }
                // If the key is equal, do nothing (BST doesn't allow duplicates)
                std::cmp::Ordering::Equal => {}
            }
            drop(current_borrow);
            Some(Rc::clone(current))
        }
        // If the node is None, create a new Node with the given key
        None => Some(new_node(key)),
    }
}

// Function to perform an inorder traversal of the BST and store the keys in a vector
// Inorder traversal ensures that the keys are stored in sorted order
fn store_sorted(node: &Option<Rc<RefCell<Node>>>, arr: &mut Vec<i32>) {
    if let Some(current) = node {
        // Traverse the left subtree
        store_sorted(&current.borrow().left, arr);
        // Store the current node's key
        arr.push(current.borrow().key);
        // Traverse the right subtree
        store_sorted(&current.borrow().right, arr);
    }
}

// Function to sort an array using Tree Sort
// Builds a BST from the array and then performs an inorder traversal to sort it
fn tree_sort(arr: &mut Vec<i32>) {
    let mut root = None;
    // Insert all elements of the array into the BST
    for &item in arr.iter() {
        root = insert(&root, item);
    }
    arr.clear(); // Clear the array to store the sorted elements
    // Perform an inorder traversal to store sorted elements back in the array
    store_sorted(&root, arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_sort() {
        let mut arr = vec![5, 4, 7, 2, 11];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![2, 4, 5, 7, 11]);
    }

    #[test]
    fn test_tree_sort_empty() {
        let mut arr = vec![];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_tree_sort_one() {
        let mut arr = vec![1];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }

    #[test]
    fn test_error() {
        let mut arr = vec![5, 4, 7, 2, 11];
        tree_sort(&mut arr);
        assert_ne!(arr, vec![4, 2, 5, 7, 12]);
    }

    #[test]
    fn test_many() {
        let mut arr = vec![5, 4, 7, 2, 11, 1, 3, 6, 8, 10, 12, 9];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn test_negative() {
        let mut arr = vec![5, 4, 7, 2, 11, -1, -3, -6, -8, -10, -12, -9];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![-12, -10, -9, -8, -6, -3, -1, 2, 4, 5, 7, 11]);
    }

    #[test]
    fn test_big_numbers() {
        let mut arr = vec![5, 4, 7, 2, 11, 100, 300, 600, 800, 1000, 1200, 900];
        tree_sort(&mut arr);
        assert_eq!(
            arr,
            vec![2, 4, 5, 7, 11, 100, 300, 600, 800, 900, 1000, 1200]
        );
    }
}
