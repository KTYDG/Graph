pub struct List {
    pub row: Vec<usize>,
}

impl List {
    pub fn push_front(&mut self, element: usize) {
        self.row.insert(0, element);
    }
    pub fn push_back(&mut self, element: usize) {
        self.row.push(element);
    }

    pub fn pop_front(&mut self) -> Option<usize> {
        let popped = Some(self.row.get(0).unwrap().clone());
        self.row.remove(0);
        popped
    }
    pub fn pop_back(&mut self) -> Option<usize> {
        self.row.pop()
    }

    pub fn get(&self) -> &Vec<usize> {
        &self.row
    }

    pub fn new_row(row: Vec<usize>) -> List {
        List { row }
    }
    pub fn new_one(element: usize) -> List {
        List { row: vec![element] }
    }
    pub fn new() -> List {
        List { row: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.row.is_empty()
    }

    pub fn exist(&self, element: usize) -> bool {
        self.row.contains(&element)
    }
}
