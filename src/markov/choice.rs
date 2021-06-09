use std::hash::Hash;

#[derive(Debug)]
#[derive(Clone, Eq, PartialEq)]
pub struct Choice<T> where T: Clone + Eq + Hash {
    pub weight: i32,
    pub value: Box<T>,
}