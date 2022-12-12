use std::{collections::HashMap, hash::Hash};

pub fn argsort<T>(elements: T) -> HashMap<T::Item, usize>
where
    T: Iterator,
    T::Item: Clone + Ord + Eq + Hash,
{
    let mut sorted: Vec<T::Item> = elements.collect();
    sorted.sort();

    let mut ans: HashMap<T::Item, usize> = Default::default();
    for i in 0..sorted.len() {
        ans.insert(sorted[i].clone(), i);
    }
    ans
}
