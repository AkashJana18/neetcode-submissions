impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        for word in strs {
            let hash = Self::word_hash(&word);
            map.entry(hash).or_default().push(word);
        }
        map.into_values().collect()
    }

    fn word_hash(word: &str) -> Vec<u8> {
        let mut hash = vec![0u8; 26];
        for c in word.bytes() {
            hash[(c - b'a') as usize] += 1;
        }
        hash
    }
}
