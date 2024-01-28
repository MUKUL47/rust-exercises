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

impl MyHashMap{
    pub fn new(max_entries: usize) -> Self{
        MyHashMap{
            data: vec![None;max_entries]
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> &mut Self{
        let char_index = MyHashMap::convert_char_usize(key) % self.data.len();
        if let Some(_) = self.get(key){
            let mut first_data = self.data[char_index].as_mut().unwrap().clone();
            if first_data.key == key {
                first_data.value = value.to_string();
                self.data[char_index] = Some(first_data);
                return self;
            }
            while first_data.next.is_some(){
                first_data = *first_data.next.unwrap();
            }
            first_data.next = Some(Box::new(MyHashMapVec{
                key: key.to_string(),
                value: value.to_string(),
                next: None
            }));
            self.data[char_index] = Some(first_data);
            return self;
        }
        self.data[char_index] = Some(MyHashMapVec{
            key: key.to_string(),
            value: value.to_string(),
            next: None
        });
        self
    }

    pub fn get(&mut self, key: &str) -> Option<String>{
        let char_index = MyHashMap::convert_char_usize(key) % self.data.len();
        if let Some(vec) = self.data[char_index].clone(){
            if vec.next.is_none() || vec.key == key{
                return Some(vec.value);
            }
            if vec.next.is_some(){
                let mut current = vec; 
                while let Some(current_next) = current.next.take(){
                    if current_next.key == key {
                        return Some(current_next.value);
                    }
                    current = *current_next;
                }
            }
        }
        None
    }

    fn convert_char_usize(key: &str) -> usize{
        let mut c: usize = 0;
        for i in key.chars().into_iter(){
            c += i as usize;
        }
        c
    }
}
