
pub mod search 
{
    use crate::trie::Trie;
    use std::collections::HashMap;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref NUMBER_MAPPING: HashMap<&'static str, i32> = {
            let mut m = HashMap::new();
            m.insert("zero", 0);
            m.insert("one", 1);
            m.insert("two", 2);
            m.insert("three", 3);
            m.insert("four", 4);
            m.insert("five", 5);
            m.insert("six", 6);
            m.insert("seven", 7);
            m.insert("eight", 8);
            m.insert("nine", 9);
            m
        };
    }

    lazy_static! {
        static ref TRIE: Trie = {
            let mut trie = Trie::new();
            trie.insert("one");
            trie.insert("two");
            trie.insert("three");
            trie.insert("four");
            trie.insert("five");
            trie.insert("six");
            trie.insert("seven");
            trie.insert("eight");
            trie.insert("nine");
            trie
        };
    }

    fn get_number_from_text(line: &str) -> Option<(i32, i32)> {
        
        let result: Option<(String, i32)> = TRIE.search_from_text(line);
        if result.is_none() {
            return None;
        }

        let text_result = result.unwrap();

        let number_from_string = text_result.0;
        let index = text_result.1;
        let number_result = NUMBER_MAPPING.get(number_from_string.as_str());

        if number_result.is_none() {
            return None
        }

        return Some((number_result.unwrap().to_owned(), index));
    }

    pub fn get_first_number_from_text(line: &str) -> Option<(i32, i32)> {
        let length = line.len() as i32;
        let mut index: usize = 0;
        let mut result = None;

        while index < length as usize {
            let temp = get_number_from_text(&line[index..]);
            
            if temp.is_none() {
                index += 1;
                continue;
            }
            
            let index_consolidated = temp.unwrap().1 + index as i32;

            if result.is_none() {
                result = Some((temp.unwrap().0, index_consolidated));
                index += 1;
                continue;
            }
            
            if (index as i32 + temp.unwrap().1) < result.unwrap().1 {
                result = Some((temp.unwrap().0, index_consolidated))
            }

            index += 1;
        }

        
        return result;

    }

    
    pub fn get_last_number_from_text(line: &str) -> Option<(i32, i32)> {
        let length = line.len() as i32;
        let mut index :i32 = 0;

        while index < length {
            let diff = (length - index) as usize;
            let result = get_number_from_text(&line[diff..]);

            if result.is_some() {
                return Some((result.unwrap().0, length - index));
            }

            index += 1;
        }
        return None;
    }
}