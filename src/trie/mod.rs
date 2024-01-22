
#[test]
fn test_add_word() {
    let mut t = Trie::new();
    t.add_word("word".to_string());
    assert!(t.is_word("word".to_string()));
}

#[test]
fn test_is_word() {
    let mut t = Trie::new();
    t.add_word("word".to_string());
    assert!(t.is_word("word".to_string()));
    assert!(!t.is_word("world1".to_string()));
}

#[test]
fn test_get_auto_complete() {
    let mut t = Trie::new();
    t.add_word("word".to_string());
    t.add_word("world".to_string());
    t.add_word("wood".to_string());
    assert_eq!(t.get_auto_complete("w".to_string()), ["wood", "word", "world"]);
}

#[test]
fn test_delete_word() {
    let mut t = Trie::new();
    t.add_word("word".to_string());
    t.add_word("world".to_string());
    t.add_word("wood".to_string());

    t.delete_word("world".to_string());
    assert!(!t.is_word("world".to_string()));
    assert!(t.is_word("wood".to_string()));
}
#[derive(Debug,Clone)]
pub struct Trie{
    data: Vec<TreeNode>,
}

#[derive(Debug,Clone)]
struct TreeNode {
    leaf_char: String,
    is_word: bool,
    sub_nodes: Vec<TreeNode>,
}
impl Trie{
    pub fn new() -> Self{
        Trie{
            data: vec![]
        }
    }
    pub fn add_word(&mut self, word: String) -> &mut Self{
       let mut some_current = Some(&mut self.data);
       for i in 0..word.len() {
            if let Some(current_data) = some_current.take(){
                let leaf_char = word.chars().nth(i).unwrap().to_string();
                let cloned_current =  current_data.clone();
                let is_next: Option<&TreeNode> = cloned_current.iter().find(|x| x.leaf_char == leaf_char);
                if is_next.is_some(){
                    let next_node = current_data.iter_mut().find(|x| x.leaf_char == leaf_char);
                    let next_node_data = next_node.unwrap();
                    if(i == word.len() - 1){
                        next_node_data.is_word = true;
                    }
                    some_current = Some(&mut next_node_data.sub_nodes);
                }else{
                    current_data.push(TreeNode{
                        is_word: i == word.len() - 1,
                        leaf_char,
                        sub_nodes: vec![]
                    });
                    let last = current_data.len() - 1;
                    some_current = Some(current_data[last].sub_nodes.as_mut());
                }
            }
        }
        self
    }

    pub fn is_word(&mut self, word: String) -> bool{
        self.get_last_char(word).map_or(false, |node| node.is_word)
    }

    fn get_last_char(&mut self, word: String) -> Option<&mut TreeNode>{
        let mut current_node = &mut self.data;
        for i in 0..word.len(){
            let leaf_char = word.chars().nth(i).unwrap().to_string();
            let next_node = current_node.iter_mut().find(|x| x.leaf_char == leaf_char);
            if next_node.is_some() {
                let next_active_node = next_node.unwrap();
                if i == word.len() - 1{
                    return Some(next_active_node);
                }
                current_node = &mut next_active_node.sub_nodes
            }else{
                return None;
            }
        }
        return None;
    }

    pub fn get_auto_complete(&mut self, prefix: String) -> Vec<String>{
        let p = prefix.clone();
        let mut final_nodes: Vec<&mut TreeNode> = vec![];
        let mut final_words: Vec<String> = vec![];
        if let Some(node) = &mut self.get_last_char(prefix).take(){
            let mut cloned_node = node.clone();
            cloned_node.leaf_char = p; 
            final_nodes.push(&mut cloned_node);
            while final_nodes.len() > 0 {
                let tree_node =final_nodes.pop().unwrap();
                for children in tree_node.sub_nodes.iter_mut(){
                    children.leaf_char = tree_node.leaf_char.clone() + &children.leaf_char.clone();
                    if children.is_word {
                        final_words.push(children.leaf_char.to_string())
                    }
                    final_nodes.push(children);
                }
            }
            return final_words
        }
        return final_words
    }

    pub fn delete_word(&mut self, word: String) -> &mut Self{
        if let Some(node) = &mut self.get_last_char(word).take(){
            node.is_word = false;
        }
        self
    }


}