use std::collections::HashMap;

pub struct Trie {
    end_of_word: bool,
    children: HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            end_of_word: false,
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(Trie::new());
        }

        node.end_of_word = true;
    }

    pub fn search_from_text(&self, line: &str) -> Option<(String, i32)> {
        let mut node = self;
        let root = self;
        let mut str_val: String = String::new();

        for (index, ch) in line.char_indices() {
            match node.children.get(&ch) {
                Some(n) => {
                    str_val.push(ch);
                    node = n;
                },
                None => {
                    
                    str_val.clear();
                    node = root;
                    
                    if node.children.contains_key(&ch){
                        str_val.push(ch);
                        node = node.children.get(&ch).unwrap();
                    }
                },
            }

            if node.end_of_word && !str_val.is_empty() {
                return Some((str_val, index as i32));
            }
        }

        return None;
    }
}