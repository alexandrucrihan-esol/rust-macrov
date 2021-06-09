use std::hash::Hash;

use crate::markov::choice::Choice;

#[derive(Debug)]
#[derive(Clone, Eq, PartialEq)]
pub struct Link<T> where T: Clone + Eq + Hash {
    pub list: Vec<Box<Choice<T>>>,
    pub total_weight: i32,
}

impl<T> Default for Link<T> where T: Clone + Eq + Hash {
    fn default() -> Self {
        Link {
            list: Vec::default(),
            total_weight: 0,
        }
    }
}
