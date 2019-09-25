// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    pub fn from_vec(vec: Vec<i32>) -> Option<Box<Self>> {
        let mut res = Self::new(0);
        for i in vec {
            res.get_last().set_next(Self::new(i));
        }
        return res.next;
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut res = vec![];
        res.push(self.val);
        let mut p = &self.next;
        while let Some(v) = p {
            res.push(v.val);
            p = &v.next;
        }
        res
    }

    pub fn set_next(&mut self, next: ListNode) {
        self.next = Some(Box::new(next))
    }

    pub fn get_last(&mut self) -> &mut Self {
        if let Some(ref mut x) = self.next {
            return x.get_last();
        }
        self
    }
}

use std::fmt;

impl fmt::Debug for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self.to_vec()))
    }
}