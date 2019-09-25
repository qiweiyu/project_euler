pub struct Queue {
    container: Vec<i32>,
    head: usize,
    tail: usize,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            container: Vec::with_capacity(10),
            head: 0,
            tail: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Queue {
            container: Vec::with_capacity(capacity),
            head: 0,
            tail: 0,
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

    fn _expand(&mut self) {
        let mut container_new = Vec::with_capacity(self.container.capacity() * 2);
        while let Some(v) = self.take() {
            container_new.push(v);
        }
        self.container = container_new;
        self.head = 0;
        self.tail = self.container.len();
    }
}