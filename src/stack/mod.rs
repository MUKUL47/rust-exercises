mod parenthesis;
#[derive(Debug, Clone)]
pub struct Stack<T> {
    data: Option<T>,
    next: Option<Box<Stack<T>>>,
}

impl<T> Stack<T> {
    pub fn new(d: T) -> Self {
        Stack { data: Some(d), next: None }
    }

    pub fn push(&mut self, data: T) {
        let mut current = self;
        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }
        current.next = Some(
            Box::new(Stack {
                data: Some(data),
                next: None,
            })
        );
    }

    pub fn peek(&mut self) -> Option<&T> {
        if let Some(d) = &mut self.next {
            return d.peek();
        } else {
            return self.data.as_ref();
        }
    }

    pub fn pop(&mut self) -> &mut Self {
        self._pop();
        return self;
    }

    fn _pop(&mut self) {
        if let Some(next_node) = &mut self.next {
            if let Some(_) = next_node.next {
                next_node._pop();
            } else {
                self.next = None;
            }
        } else {
            self.next = None;
            self.data = None;
        }
    }

    pub fn get_by_idx(&mut self, index: i32) -> Option<&T> {
        if index == 0 {
            return self.data.as_ref();
        }
        if let Some(next_node) = &mut self.next {
            return next_node.get_by_idx(index - 1);
        } else {
            return None;
        }
    }

    pub fn iter<F: Fn(Option<&T>, i32) + Copy>(&mut self, closure: F) {
        let mut index = 0;
        let mut current = self;
        while current.next.is_some() {
            closure(current.data.as_ref(), index);
            index += 1;
            current = current.next.as_mut().unwrap();
        }
        if let Some(_) = current.data {
            closure(current.data.as_ref(), index);
        }
    }
}
