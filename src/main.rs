mod utils;
mod fraction;
mod primes;
mod fib;
mod queue;

struct Node {
    val: i32,
    is_val: bool,
    is_end: bool,
    forward: i32,
    backward: i32,
    next: Option<Vec<Box<Node>>>,
}

impl Node {
    fn find_not_end_leaves(&self) -> Option<Vec<&Node>> {
        if self.is_end {
            return None;
        }
        match &self.next {
            None => Some(vec![self]),
            Some(list) => {
                let mut res = vec![];
                for n in list {
                    if let Some(sub_list) = n.find_not_end_leaves() {
                        for leaf in sub_list {
                            res.push(leaf);
                        }
                    }
                }
                if res.len() > 0 {
                    Some(res)
                } else {
                    None
                }
            }
        }
    }
}

struct Tree {
    head: Box<Node>
}

impl Tree {
    fn new(val: i32) -> Self {
        Tree {
            head: Box::new(Node {
                val,
                is_val: false,
                is_end: false,
                forward: val,
                backward: val,
                next: None,
            })
        }
    }

    fn generate(&mut self) -> i32 {
        0
    }

    fn find_not_end_leaves(&self) -> Option<Vec<&Node>> {
        self.head.find_not_end_leaves()
    }
}

fn main() {}