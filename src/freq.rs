use std::collections::HashSet;
use std::collections::HashMap;
use std::string::ToString;
use regex::Regex;

pub type FreqMap = HashMap<char, u32>;

fn inc_freq_map_entry(map: &mut FreqMap, key: char, inc: u32) {
    let curr = map.get(&key).unwrap_or(&0).clone();
    map.insert(key, curr + inc);
}

pub fn merge_freq_maps(dst: &mut FreqMap, src: &FreqMap) {
    for (key, value) in src.iter() {
        inc_freq_map_entry(dst, *key, *value)
    }
}

pub struct FreqGetter {
    filter_regex: Option<Regex>,
    filter_set: Option<HashSet<char>>
}

impl FreqGetter {
    pub fn new() -> Self {
        FreqGetter {
            filter_regex: None,
            filter_set: None,
        }
    }

    pub fn filter_regex(mut self, regex: Regex) -> Self {
        self.filter_regex = Some(regex);
        self
    }

    pub fn filter_set(mut self, set: HashSet<char>) -> Self {
        self.filter_set = Some(set);
        self
    }

    fn filter_char(&self, c: char) -> bool {
        match (&self.filter_regex,
               &self.filter_set) {
            (Some(regex), ..) => regex.find(&c.to_string()).is_some(),
            (None, Some(set)) => set.contains(&c),
            _ => true,
        }
    }

    pub fn get(&self, input: &str) -> FreqMap {
        let mut out = FreqMap::new();

        input
            .chars()
            .filter(|c| self.filter_char(c.clone()))
            .for_each(|c| {
                let curr = out.get(&c).unwrap_or(&0).clone();
                out.insert(c, curr + 1);
            });

        out
    }
}

pub fn get_freq(input: &str) -> FreqMap {
    let getter = FreqGetter::new();
    getter.get(input)
}

#[test]
fn test_get_freq() {
    let input = "araarrrqqqwqqqq";
    let freq = get_freq(&input);
    assert_eq!(freq.get(&'a'), Some(&3));
    assert_eq!(freq.get(&'r'), Some(&4));
    assert_eq!(freq.get(&'q'), Some(&7));
    assert_eq!(freq.get(&'w'), Some(&1));
}

#[test]
fn test_filter_set() {
    let input = "araarrrqqqwqyyyyyqqq   ";

    let filter_set: HashSet<char> = vec!['a', 'r'].into_iter().collect();

    let freq =
        FreqGetter::new()
        .filter_set(filter_set)
        .get(&input);

    assert_eq!(freq.get(&'a'), Some(&3));
    assert_eq!(freq.get(&'r'), Some(&4));
    assert_eq!(freq.get(&'q'), None);
    assert_eq!(freq.get(&'w'), None);
    assert_eq!(freq.get(&' '), None);
}

#[test]
fn test_filter_regex() {
    let input = "araarrrqqqwqyyyyyqqq   ";

    let regex = Regex::new(r"\w").unwrap();

    let freq =
        FreqGetter::new()
        .filter_regex(regex)
        .get(&input);

    assert_eq!(freq.get(&'a'), Some(&3));
    assert_eq!(freq.get(&'r'), Some(&4));
    assert_eq!(freq.get(&'q'), Some(&7));
    assert_eq!(freq.get(&'w'), Some(&1));
    assert_eq!(freq.get(&' '), None);
}

#[test]
fn test_merge() {
    let input1 = "araarrrqqqwqyyyyyqqq";
    let input2 = "uuuuuuuuuuuuuuuuuuuuuuuqqq";
    let mut freq1 = get_freq(&input1);
    let freq2 = get_freq(&input2);
    merge_freq_maps(&mut freq1, &freq2);

    assert_eq!(freq1.get(&'a'), Some(&3));
    assert_eq!(freq1.get(&'r'), Some(&4));
    assert_eq!(freq1.get(&'q'), Some(&10));
    assert_eq!(freq1.get(&'w'), Some(&1));
    assert_eq!(freq1.get(&'u'), Some(&23));
}
