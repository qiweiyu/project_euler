pub struct Queue {
    container: Vec<i32>,
    head: usize,
    tail: usize,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            container: vec![0; 10],
            head: 0,
            tail: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Queue {
            container: vec![0; capacity],
            head: 0,
            tail: 0,
        }
    }

    pub fn from_vec(from: Vec<i32>) -> Self {
        let len = from.len();
        let mut container_new = vec![0; len+1];
        for i in 0..len {
            container_new[i] = from[i];
        }
        Queue {
            container: container_new,
            head: 0,
            tail: len,
        }
    }

    pub fn put(&mut self, v: i32) {
        let mut next_tail = (self.tail + 1) % self.container.capacity();
        if next_tail == self.head {
            self._expand();
            next_tail = self.tail + 1;
        }
        self.container[self.tail] = v;
        self.tail = next_tail;
    }

    pub fn take(&mut self) -> Option<i32> {
        if self.head == self.tail {
            None
        } else {
            let cur_head = self.head;
            self.head = (self.head + 1) % self.container.capacity();
            Some(self.container[cur_head])
        }
    }

    pub fn len(&self) -> usize {
        (self.tail + self.capacity() - self.head) % self.capacity()
    }

    pub fn capacity(&self) -> usize {
        self.container.capacity()
    }

    pub fn into_vec(mut self) -> Vec<i32> {
        let mut res = vec![];
        while let Some(v) = self.take() {
            res.push(v);
        }
        res
    }

    fn _expand(&mut self) {
        let mut container_new = vec![0; self.capacity() * 2];
        let mut i = 0;
        while let Some(v) = self.take() {
            container_new[i] = v;
            i += 1;
        }
        self.container = container_new;
        self.head = 0;
        self.tail = i;
    }
}

#[test]
fn test_queue() {
    let q = Queue::new();
    assert_eq!(q.len(), 0);
    assert_eq!(q.capacity(), 10);
    let q = Queue::with_capacity(25);
    assert_eq!(q.len(), 0);
    assert_eq!(q.capacity(), 25);
    let mut q = Queue::with_capacity(4);
    q.put(29);
    q.put(31);
    q.put(37);
    assert_eq!(q.len(), 3);
    assert_eq!(q.capacity(), 4);
    let mut q = Queue::with_capacity(4);
    q.put(29);
    q.put(31);
    q.put(37);
    assert_eq!(q.take(), Some(29));
    q.put(41);
    q.put(43);
    assert_eq!(q.len(), 4);
    assert_eq!(q.capacity(), 8);
    assert_eq!(q.take(), Some(31));
    assert_eq!(q.take(), Some(37));
    assert_eq!(q.take(), Some(41));
    assert_eq!(q.take(), Some(43));
    let mut q = Queue::from_vec(vec![2, 5, 7, 11]);
    assert_eq!(q.len(), 4);
    assert_eq!(q.take(), Some(2));
    assert_eq!(q.take(), Some(5));
    q.put(13);
    q.put(17);
    assert_eq!(q.into_vec(), vec![7, 11, 13, 17]);
}