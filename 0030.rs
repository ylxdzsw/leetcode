// You are given a string, s, and a list of words, words, that are all of the same length. Find all starting indices of substring(s) in s that is a concatenation of each word in words exactly once and without any intervening characters.

// Example 1:

// Input:
//   s = "barfoothefoobarman",
//   words = ["foo","bar"]
// Output: [0,9]
// Explanation: Substrings starting at index 0 and 9 are "barfoo" and "foobar" respectively.
// The output order does not matter, returning [9,0] is fine too.
// Example 2:

// Input:
//   s = "wordgoodgoodgoodbestword",
//   words = ["word","good","best","word"]
// Output: []

fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    if s.is_empty() || words.is_empty() {
        return vec![]
    }

    let nwords = words.len();
    let wlen = words[0].len();
    let mut counts: Vec<usize> = vec![];
    let mut tree = std::collections::BTreeMap::new();
    for word in words {
        let i = tree.entry(Box::leak(Box::new(word.into_bytes())).as_slice()).or_insert_with(|| { counts.push(0); counts.len() - 1});
        counts[*i] += 1;
    }

    fn _find_substring(s: &[u8], tree: &std::collections::BTreeMap<&[u8], usize>, target_counts: &[usize], wlen: usize, nwords: usize) -> Vec<i32> {
        let indices: Vec<_> = s.chunks_exact(wlen).map(|w| tree.get(w)).collect();
        indices.windows(nwords).enumerate().flat_map(|(i, window)| {
            let mut counts = vec![0; target_counts.len()];
            for &x in window {
                counts[*x?] += 1
            }
            if counts == target_counts {
                Some(i as _)
            } else {
                None
            }
        }).collect()
    }

    let s = s.as_bytes();
    let mut result = vec![];
    for offset in 0..wlen {
        for i in _find_substring(&s[offset..], &tree, &counts, wlen, nwords) {
            result.push(offset as i32 + i * wlen as i32)
        }
    }

    result
}

fn main() {
    assert_eq!(find_substring("barfoothefoobarman".to_string(), vec!["foo".to_string(),"bar".to_string()]), vec![0, 9]);
    assert_eq!(find_substring("wordgoodgoodgoodbestword".to_string(), vec![
        "word".to_string(), "good".to_string(), "best".to_string(), "word".to_string()
    ]), vec![]);
}

// faster than 83.33%, less memory than 100.00%.