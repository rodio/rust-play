use std::collections::VecDeque;

fn main() {
    let tree = Tree::from_level_order(vec![1, 2, 3, 4, 5, 6]);
    dbg!(tree);
}

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn from_level_order(level_order_vector: Vec<i32>) -> Self {
        if level_order_vector.is_empty() {
            return Self { root: None };
        }

        let mut root = &mut Some(Box::new(Node {
            value: level_order_vector[0],
            left: None,
            right: None,
        }));

        let mut queue = VecDeque::new();
        queue.push_back(root);

        let mut i = 1;
        while i < level_order_vector.len() {
            let option_box = queue.pop_front().unwrap();
            let curr_mut_box = option_box.as_mut().unwrap();

            // if i < level_order_vector.len() {
            curr_mut_box.left = Some(Box::new(Node {
                value: level_order_vector[i],
                left: None,
                right: None,
            }));
            queue.push_back(&mut curr_mut_box.left);
            i += 1;
            // }

            if i < level_order_vector.len() {
                curr_mut_box.right = Some(Box::new(Node {
                    value: level_order_vector[i],
                    left: None,
                    right: None,
                }));
                queue.push_back(&mut curr_mut_box.right);
                i += 1;
            }
        }

        Self { root }
    }
}
