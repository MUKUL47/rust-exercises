pub struct Parenthesis;
impl Parenthesis {
    pub fn validate(s: &str) -> bool {
        let mut stack: Vec<char> = vec![];
        for c in s.chars().into_iter() {
            match c {
                '{' | '}' | '[' | ']' => {
                    if stack.len() > 0 && *stack.get(stack.len() - 1).unwrap() == c {
                        stack.pop();
                        continue;
                    }
                    stack.push(c);
                }
                _ => {}
            }
        }
        return stack.len() == 0;
    }
}
