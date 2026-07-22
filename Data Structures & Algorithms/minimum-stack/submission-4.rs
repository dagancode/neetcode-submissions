
struct MinStack {
    inner: Vec<i32>,
    min: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            min: vec![i32::MAX],
        }
    }

    pub fn push(&mut self, val: i32) {
        if val < self.get_min() {
            self.min.push(val);
        } else {
            self.min.push(self.get_min());
        }
        self.inner.push(val);
    }

    pub fn pop(&mut self) {
        self.inner.pop();
        self.min.pop();
    }

    pub fn top(&self) -> i32 {
        *self.inner.last().expect("requires at least 1 item")
    }

    pub fn get_min(&self) -> i32 {
        *self.min.last().expect("requires at least 1 item")
    }
}
