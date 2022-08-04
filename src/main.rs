use std::collections::{BTreeSet, HashSet};

const WORDS: &str = include_str!("five-letter-words.txt");

fn main() {
    let words: HashSet<&str> = WORDS.lines().collect();
    let packed_words: BTreeSet<u32> = words.iter().copied().flat_map(word_to_num).collect();
    let packed_words: Vec<u32> = packed_words.into_iter().collect();
    println!(
        "found {} unique sets of 5 letters that appear in words",
        packed_words.len()
    );
    let mut results: BTreeSet<(u32, u32, u32, u32, u32)> = BTreeSet::new();
    for (i1, w1) in packed_words.iter().copied().enumerate() {
        let ti = i1 + 1;
        let wi = w1;
        for (i2, w2) in packed_words[ti..].iter().copied().enumerate() {
            if (wi & w2) != 0 {
                continue;
            }
            let ti = ti + i2;
            let wi = wi | w2;
            for (i3, w3) in packed_words[ti..].iter().copied().enumerate() {
                if (wi & w3) != 0 {
                    continue;
                }
                let ti = ti + i3;
                let wi = wi | w3;
                for (i4, w4) in packed_words[ti..].iter().copied().enumerate() {
                    if (wi & w4) != 0 {
                        continue;
                    }
                    let ti = ti + i4;
                    let wi = wi | w4;
                    for w5 in packed_words[ti..].iter() {
                        if (wi & w5) == 0 {
                            results.insert((w1, w2, w3, w4, *w5));
                        }
                    }
                }
            }
        }
    }
    println!(
        "{} five word anagram groups with no overlapping letters found",
        results.len()
    );
    for (w1, w2, w3, w4, w5) in results {
        for word1 in get_words_for_num(w1, &words) {
            for word2 in get_words_for_num(w2, &words) {
                for word3 in get_words_for_num(w3, &words) {
                    for word4 in get_words_for_num(w4, &words) {
                        for word5 in get_words_for_num(w5, &words) {
                            println!("{word1} {word2} {word3} {word4} {word5}");
                        }
                    }
                }
            }
        }
    }
}

fn word_to_num(word: &str) -> Option<u32> {
    let mut res = 0;
    for c in word.bytes() {
        let letter_bit = 1 << (c - b'a');
        if (res & letter_bit) != 0 {
            return None;
        }
        res |= letter_bit;
    }
    Some(res)
}

fn get_words_for_num<'a>(num: u32, words: &'a HashSet<&'a str>) -> impl Iterator<Item = &'a str> {
    words
        .iter()
        .copied()
        .filter(move |word| word_to_num(word) == Some(num))
}
