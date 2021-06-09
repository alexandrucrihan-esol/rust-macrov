use std::fmt::{Debug, Error, Formatter};
use std::hash::Hash;

use crate::markov::graph::Graph;
use crate::markov::link::Link;

pub struct Traversal<'a, T, U>
    where T: Clone + Eq + Hash,
          U: Fn() -> f32 {
    current: T,
    generator: U,
    graph: &'a Graph<T>,
}

impl<'a, T, U> Traversal<'a, T, U>
    where T: Clone + Eq + Hash + Debug,
          U: Fn() -> f32 {
    pub fn new(current: T, generator: U, graph: &'a Graph<T>) -> Traversal<T, U> {
        Traversal {
            current,
            generator,
            graph,
        }
    }


    pub fn choose(&mut self) -> Option<T> {
        let option = self.graph.value_to_link.get(&self.current);
        if option == None {
            return None;
        }

        let Link { list, total_weight } = option.unwrap();

        let random = (self.generator)();
        let threshold = (random * (*total_weight) as f32) as i32;

        let mut result = list.get(0)
            .cloned()
            .map(|e| e.value);

        let mut iter = list.iter();

        while let Some(e) = iter.next() {
            if e.weight <= threshold {
                let value = e.clone().value;
                result = Some(value);
            } else {
                break;
            }
        }

        match result.as_deref().clone() {
            Some(e) => {
                let value = e.clone();
                self.current = value.clone();
                Some(value)
            }
            None => None
        }
    }
}

impl<'a, T, U> Debug for Traversal<'a, T, U>
    where T: Clone + Eq + Hash + Debug,
          U: Fn() -> f32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Traversal")
            .field("current", &self.current)
            .field("graph", &self.graph)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::markov::graph::Graph;

    #[test]
    fn traversal_generator_under_returns_lowest() {
        let mut graph = Graph::default();

        graph.new_edge(Box::from(2))
            .towards(15, Box::from(15))
            .towards(10, Box::from(10))
            .towards(5, Box::from(5));

        let mut traversal = graph.new_traversal(
            Box::from(2),
            || { 0.0 },
        );

        let result = &traversal.choose();

        let expected_current = Box::from(5);
        assert_eq!(result, &Some(expected_current.clone()));
        assert_eq!(traversal.current, expected_current.clone());
    }

    #[test]
    fn traversal_generator_after_first_returns_next() {
        let mut graph = Graph::default();
        graph.new_edge(Box::from(2))
            .towards(15, Box::from(15))
            .towards(10, Box::from(10))
            .towards(5, Box::from(5));

        let mut traversal = graph.new_traversal(
            Box::from(2),
            || { 0.9 },
        );

        let result = &traversal.choose();

        let expected_current = Box::from(15);
        assert_eq!(result, &Some(expected_current.clone()));
        assert_eq!(traversal.current, expected_current.clone());
    }

    #[test]
    fn traversal_generator_after_first_returns_middle() {
        let mut graph = Graph::default();
        graph.new_edge(Box::from(2))
            .towards(15, Box::from(15))
            .towards(10, Box::from(10))
            .towards(5, Box::from(5));

        let mut traversal = graph.new_traversal(
            Box::from(2),
            || { 0.4 },
        );
        let result = &traversal.choose();

        let expected_current = Box::from(10);
        assert_eq!(result, &Some(expected_current.clone()));
        assert_eq!(traversal.current, expected_current.clone());
    }

    #[test]
    fn traversal_generator_current_no_values_returns_none() {
        let mut graph = Graph::default();

        graph.new_edge(Box::from(2))
            .towards(15, Box::from(15))
            .towards(10, Box::from(10))
            .towards(5, Box::from(5));

        let mut traversal = graph.new_traversal(
            Box::from(30),
            || { 0.9 },
        );
        let result = &traversal.choose();

        assert_eq!(result, &None);
        assert_eq!(traversal.current, Box::from(30));
    }
}