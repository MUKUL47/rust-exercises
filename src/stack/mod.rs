
#[derive(Debug,Clone)]
pub struct Stack{
    stack_pointer: usize,
    data: [usize;10],
}

impl  Stack{
    pub fn new() -> Self {
        Stack {
            data: Default::default(),
            stack_pointer: 0,
        }
    }

    pub fn push(&mut self, data: usize) -> &mut Self{
        if self.stack_pointer == self.data.len() {
            return self;
        }
        self.data[self.stack_pointer] = data;
        self.stack_pointer += 1 ;
        return self;
    }

    pub fn pop(&mut self) -> Option<usize> {
        if self.stack_pointer == 0 {
            return None;
        }
        self.stack_pointer -= 1;
        return Some(self.data[self.stack_pointer]);
    }

    pub fn get(&self) -> &[usize] {
        return &self.data[0..self.stack_pointer]
    }

    pub fn peek(&self) -> Option<&usize> {
        if self.stack_pointer == 0 {
            return None;
        }
        return Some(&self.data[self.stack_pointer - 1]);
    }

    pub fn size(&self) -> usize{
        return self.stack_pointer;
    }

}