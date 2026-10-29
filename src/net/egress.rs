pub struct Egress {
    list: Vec<String>,
    cursor: usize,
}

impl Egress {
    pub fn new(list: Vec<String>) -> Self {
        Self { list, cursor: 0 }
    }

    pub fn next(&mut self) -> Option<String> {
        if self.list.is_empty() {
            return None;
        }
        let value = self.list[self.cursor % self.list.len()].clone();
        self.cursor += 1;
        Some(value)
    }
}
