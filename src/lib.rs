use std::{collections::HashMap, str::Chars};

#[derive(Debug, PartialEq)]
struct Node {
    char: char,
    is_word: bool,
    children: HashMap<char, Node>,
}

#[derive(Debug)]
pub struct Trie {
    head_node: Node,
}

impl Trie {
    pub fn new() -> Trie {
        return Trie {
            head_node: Node {
                char: ' ',
                is_word: false,
                children: HashMap::new(),
            },
        };
    }

    pub fn add_word(&mut self, word: &str) {
        let mut current_node = &mut self.head_node;
        let mut index = 0;

        let len = word.chars().count();

        for letter in word.chars() {
            current_node = current_node.children.entry(letter).or_insert(Node {
                char: letter,
                is_word: index == len - 1,
                children: HashMap::new(),
            });

            index += 1;
        }
    }

    pub fn get_suggestions(&self, prefix: &str) -> Vec<String> {
        let Some(prefix_node) = Self::find_prefix_node(&self.head_node, prefix.chars()) else {
            return Vec::new();
        };

        let mut path = prefix.to_string();
        path.pop();
        let mut suggestions = vec![];

        Self::dfs_words(prefix_node, path, &mut suggestions);
        return suggestions;
    }

    fn find_prefix_node<'a>(previous_node: &'a Node, mut chars: Chars) -> Option<&'a Node> {
        let Some (letter) = chars.next() else {
            return Some(previous_node);
        };

        let Some (next_node) = previous_node.children.get(&letter) else {
            return None
        };

        return Self::find_prefix_node(next_node, chars);
    }

    fn dfs_words(node: &Node, mut path: String, suggestions: &mut Vec<String>) {
        path.push(node.char);

        if node.is_word {
            suggestions.push(path.clone())
        }

        for (_, child) in node.children.iter() {
            Self::dfs_words(child, path.clone(), suggestions);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{Node, Trie};

    #[test]
    fn trie_can_receive_words() {
        let mut trie = Trie::new();

        trie.add_word("te");

        let children = map_node(Node {
            char: 't',
            is_word: false,
            children: map_node(Node {
                char: 'e',
                is_word: true,
                children: HashMap::new(),
            }),
        });

        assert_eq!(trie.head_node, base_node(children));

        trie.add_word("tes");

        let children = map_node(Node {
            char: 't',
            is_word: false,
            children: map_node(Node {
                char: 'e',
                is_word: true,
                children: map_node(Node {
                    char: 's',
                    is_word: true,
                    children: HashMap::new(),
                }),
            }),
        });

        assert_eq!(trie.head_node, base_node(children));
    }

    #[test]
    fn trie_can_provide_suggestions() {
        let mut trie = Trie::new();

        trie.add_word("car");
        trie.add_word("truck");
        trie.add_word("train");

        let mut suggestions = trie.get_suggestions("t");
        suggestions.sort_unstable();
        assert_eq!(suggestions, vec!["train", "truck"]);

        let mut suggestions = trie.get_suggestions("tr");
        suggestions.sort_unstable();
        assert_eq!(suggestions, vec!["train", "truck"]);

        let mut suggestions = trie.get_suggestions("tru");
        suggestions.sort_unstable();
        assert_eq!(suggestions, vec!["truck"]);

        let mut suggestions = trie.get_suggestions("pla");
        suggestions.sort_unstable();
        assert_eq!(suggestions.len(), 0);
    }

    #[inline(always)]
    fn map_node(node: Node) -> HashMap<char, Node> {
        return HashMap::from([(node.char, node)]);
    }

    #[inline(always)]
    fn base_node(children: HashMap<char, Node>) -> Node {
        return Node {
            char: ' ',
            is_word: false,
            children,
        };
    }
}
