pub enum Family {
    None,
    Last { item: usize },
    Node { item: usize, next: Box<Family> },
}

impl Family {
    pub fn new() -> Self {
        Self::None
    }

    pub fn push(&mut self, item: usize) {
        match self {
            Self::None => self.to_last(item),
            Self::Last { .. } => self.to_node(item),
            Self::Node { next, .. } => next.push(item),
        }
    }

    fn to_last(&mut self, item: usize) {
        *self = match self {
            Self::None => Self::Last { item },
            Self::Node { item: _, next: _ } => Self::Last { item },
            _ => panic!("Where are you going??"),
        }
    }

    fn to_node(&mut self, it: usize) {
        *self = match self {
            Self::Last { item } => Self::Node {
                item: *item,
                next: Box::new(Self::Last { item: it }),
            },
            _ => panic!("Can't make node"),
        }
    }

    fn delete(&mut self) {
        // *self = std::mem::replace(self, Family::None);
        *self = Family::None;
    }
    fn next(&mut self, next: Family) {
        *self = next;
    }

    pub fn pop(&mut self) -> Option<usize> {
        match self {
            Self::None => None,
            Self::Last { item } => {
                let item = *item;
                self.delete();
                Some(item)
            }
            Self::Node { item, next } => {
                let mut n = Box::new(Self::None);
                let item = *item;
                std::mem::swap(next, &mut n);
                self.next(*n);
                Some(item)
            }
        }
    }

    pub fn path(&mut self, reverse: bool) -> String {
        let mut path = String::new();
        let mut node = self.pop();
        while node != None {
            if path.len() == 0 {
                path = format!("{}", node.unwrap());
            } else {
                if reverse {
                    path = format!("{}>-{}", path, node.unwrap());
                } else {
                    path = format!("{}->{}", path, node.unwrap());
                }
            }
            node = self.pop();
        }
        if reverse {
            path.chars().rev().collect::<String>()
        } else {
            path
        }
    }
}

pub fn print_path(path: Vec<usize>) -> String {
    let mut str = String::new();
    for element in path {
        if str.len() == 0 {
            str = format!("{}", element);
        } else {
            str = format!("{}->{}", str, element);
        }
    }
    str
}
