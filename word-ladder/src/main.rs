/*
A transformation sequence from word beginWord to word endWord using a dictionary wordList is a
sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:

    Every adjacent pair of words differs by a single letter.  Every si for 1 <= i <= k is in
    wordList. Note that beginWord does not need to be in wordList.  sk == endWord

Given two words, beginWord and endWord, and a dictionary wordList, return the number of words in
the shortest transformation sequence from beginWord to endWord, or 0 if no such sequence exists.
*/

trait Solution {}
use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let len = begin_word.len();
        let mut word_map: HashMap<String, Vec<&str>> =
            HashMap::with_capacity(word_list.len() * len);
        word_list.iter().map(String::as_str).for_each(|word| {
            for i in 0..len {
                let generic = format!("{}*{}", &word[0..i], &word[i + 1..len]);
                word_map
                    .entry(generic)
                    .and_modify(|v| v.push(word))
                    .or_insert(vec![word]);
            }
        });

        let mut queue: VecDeque<(&str, i32)> = VecDeque::new();
        queue.push_back((&begin_word, 1));

        let mut visited: HashSet<&str> = HashSet::new();
        visited.insert(&begin_word);

        while let Some((word, level)) = queue.pop_front() {
            for i in 0..len {
                let new_word = format!("{}*{}", &word[0..i], &word[i + 1..len]);
                for adjacent in word_map.get(&new_word).unwrap_or(&vec![]) {
                    if adjacent == &end_word {
                        return level + 1;
                    }
                    if !visited.contains(adjacent) {
                        visited.insert(adjacent);
                        queue.push_back((adjacent, level + 1));
                    }
                }
            }
        }

        0
    }
}

fn main() {}
