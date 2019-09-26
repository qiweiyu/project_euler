pub struct Queue<T> {
    container: Vec<T>,
    head: usize,
    tail: usize,
}

impl<T> Queue<T>
    where T: Default + Clone
{
    pub fn new() -> Self {
        Queue {
            container: vec![T::default(); 10],
            head: 0,
            tail: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Queue {
            container: vec![T::default(); capacity],
            head: 0,
            tail: 0,
        }
    }

    pub fn from_vec(from: Vec<T>) -> Self {
        let len = from.len();
        let mut container_new = vec![T::default(); len + 1];
        for i in 0..len {
            container_new[i] = from[i].clone();
        }
        Queue {
            container: container_new,
            head: 0,
            tail: len,
        }
    }

    pub fn insert(&mut self, v: T) {
        let mut next_tail = (self.tail + 1) % self.container.capacity();
        if next_tail == self.head {
            self._expand();
            next_tail = self.tail + 1;
        }
        self.container[self.tail] = v;
        self.tail = next_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head == self.tail {
            None
        } else {
            let cur_head = self.head;
            self.head = (self.head + 1) % self.container.capacity();
            Some(self.container[cur_head].clone())
        }
    }

    pub fn len(&self) -> usize {
        (self.tail + self.capacity() - self.head) % self.capacity()
    }

    pub fn capacity(&self) -> usize {
        self.container.capacity()
    }

    pub fn into_vec(mut self) -> Vec<T> {
        let mut res = vec![];
        while let Some(v) = self.pop() {
            res.push(v);
        }
        res
    }

    fn _expand(&mut self) {
        let mut container_new = vec![T::default(); self.capacity() * 2];
        let mut i = 0;
        while let Some(v) = self.pop() {
            container_new[i] = v;
            i += 1;
        }
        self.container = container_new;
        self.head = 0;
        self.tail = i;
    }
}

impl<T> Default for Queue<T>
    where T: Default + Clone
{
    fn default() -> Queue<T> {
        Queue {
            container: vec![T::default(); 10],
            head: 0,
            tail: 0,
        }
    }
}

impl<T> Iterator for Queue<T>
    where T: Default + Clone
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

#[test]
fn test_queue() {
    let q: Queue<i32> = Queue::new();
    assert_eq!(q.len(), 0);
    assert_eq!(q.capacity(), 10);
    let q: Queue<i32> = Queue::with_capacity(25);
    assert_eq!(q.len(), 0);
    assert_eq!(q.capacity(), 25);
    let mut q = Queue::with_capacity(4);
    q.insert(29);
    q.insert(31);
    q.insert(37);
    assert_eq!(q.len(), 3);
    assert_eq!(q.capacity(), 4);
    let mut q = Queue::with_capacity(4);
    q.insert(29);
    q.insert(31);
    q.insert(37);
    assert_eq!(q.pop(), Some(29));
    q.insert(41);
    q.insert(43);
    assert_eq!(q.len(), 4);
    assert_eq!(q.capacity(), 8);
    assert_eq!(q.pop(), Some(31));
    assert_eq!(q.pop(), Some(37));
    assert_eq!(q.pop(), Some(41));
    assert_eq!(q.pop(), Some(43));
    let mut q = Queue::from_vec(vec![2, 5, 7, 11]);
    assert_eq!(q.len(), 4);
    assert_eq!(q.pop(), Some(2));
    assert_eq!(q.pop(), Some(5));
    q.insert(13);
    q.insert(17);
    assert_eq!(q.into_vec(), vec![7, 11, 13, 17]);
    let mut q = Queue::from_vec(vec![2, 5, 7, 11]);
    let mut i = q.into_iter();
    assert_eq!(i.next(), Some(2));
    assert_eq!(i.next(), Some(5));
    assert_eq!(i.next(), Some(7));
    assert_eq!(i.next(), Some(11));
    assert_eq!(i.next(), None);
}