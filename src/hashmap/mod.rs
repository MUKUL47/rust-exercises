
impl MyHashMap{
    pub fn new(max_entries: usize) -> Self{
        MyHashMap{
            data: vec![None;max_entries]
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> &mut Self{
        let char_index = MyHashMap::convert_char_usize(key) % self.data.len();
        match self._has(key){
            HashMap::NoKey => {
                self.data[char_index] = Some(MyHashMapVec{
                    key: key.to_string(),
                    value: value.to_string(),
                    next: None
                });
            },
            _ => {
                let mut first_data = self.data[char_index].as_mut().unwrap();
                while first_data.next.is_some(){
                    first_data = first_data.next.as_mut().unwrap();
                }
                first_data.next = Some(Box::new(MyHashMapVec{
                    key: key.to_string(),
                    value: value.to_string(),
                    next: None
                }));
                return self;
            }
        }
        self
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        match self._has(key){
            HashMap::HasKey(value) => Some(value),
            _ => None
        }
    }

    pub fn has(&mut self, key: &str) -> bool {
        match self._has(key){
            HashMap::HasKey(_) => true,
            _ => false
        }
    }

   fn _has(&mut self, key: &str) -> HashMap {
        let char_index = MyHashMap::convert_char_usize(key) % self.data.len();
        if let Some(vec) = self.data[char_index].clone(){
            if vec.next.is_none() || vec.key == key{
                return HashMap::HasKey(vec.value.to_string());
            }
            if vec.next.is_some(){
                let mut current = vec; 
                while let Some(current_next) = current.next.take(){
                    if current_next.key == key {
                        return HashMap::HasKey(current_next.value.to_string());
                    }
                    current = *current_next;
                }
                return HashMap::HasCollided;
            }
        }
        return HashMap::NoKey;
    }

    fn convert_char_usize(key: &str) -> usize{
        let mut c: usize = 0;
        for i in key.chars().into_iter(){
            c += i as usize;
        }
        c
    }
}

enum HashMap{
    HasKey(String),
    HasCollided,
    NoKey
}
#[derive(Debug,Clone)]
pub struct MyHashMap{
    data: Vec<Option<MyHashMapVec>>
}
#[derive(Debug,Clone)]
struct MyHashMapVec{
    key: String,
    value: String,
    next: Option<Box<MyHashMapVec>>
}
