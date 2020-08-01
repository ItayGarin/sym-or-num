use std::collections::HashSet;
use std::collections::HashMap;
use std::string::ToString;
use regex::Regex;

type FreqMap = HashMap<char, u32>;

struct FreqGetter {
    filter_fn: Option<Box<dyn Fn(char) -> bool>>,
    filter_regex: Option<Regex>,
    filter_set: Option<HashSet<char>>
}

impl FreqGetter {
    fn new() -> Self {
        FreqGetter {
            filter_fn: None,
            filter_regex: None,
            filter_set: None,
        }
    }

    pub fn filter_fn(mut self, fun: impl Fn(char) -> bool + 'static) -> Self {
        self.filter_fn = Some(Box::new(fun));
        self
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
        match (&self.filter_fn,
               &self.filter_regex,
               &self.filter_set) {
            (Some(fun), ..) => fun(c),
            (None, Some(regex), ..) => regex.find(&c.to_string()).is_some(),
            (None, None, Some(set)) => set.contains(&c),
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

fn get_freq(input: &str) -> FreqMap {
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
fn test_filter_fn() {
    let input = "araarrrqqqwqyyyyyqqq   ";

    let freq =
        FreqGetter::new()
        .filter_fn(|c| c != ' ')
        .get(&input);

    assert_eq!(freq.get(&'a'), Some(&3));
    assert_eq!(freq.get(&'r'), Some(&4));
    assert_eq!(freq.get(&'q'), Some(&7));
    assert_eq!(freq.get(&'w'), Some(&1));
    assert_eq!(freq.get(&' '), None);
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
