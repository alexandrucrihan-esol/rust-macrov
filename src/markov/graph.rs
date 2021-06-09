use std::collections::HashMap;
use std::hash::Hash;

use crate::markov::edge::Edge;
use crate::markov::link::Link;
use crate::markov::traversal::Traversal;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Graph<T> where T: Clone + Eq + Hash {
    pub value_to_link: HashMap<Box<T>, Link<T>>,
}

impl<T> Default for Graph<T> where T: Clone + Eq + Hash {
    fn default() -> Self {
        Graph {
            value_to_link: HashMap::new()
        }
    }
}

impl<T: Clone + Eq + Hash + Debug> Graph<T> {
    pub fn new_edge(&mut self, value: T) -> Edge<T> {
        Edge { value_to_link: &mut self.value_to_link, value: Box::from(value) }
    }

    pub fn new_traversal<F: Fn() -> f32>(&mut self, start_value: T, generator: F) -> Traversal<T, F> {
        Traversal::new(start_value, generator, self)
    }
}


#[cfg(test)]
mod tests {
    use crate::markov::graph::Graph;

    #[test]
    fn hash_with_boxed_key_modifies_same_value() {
        let mut graph = Graph::default();

        let mut edge = graph.new_edge("Start");
        edge.towards(2, "Corridor");
        edge.towards(1, "Corner");

        let mut edge = graph.new_edge("Corridor");
        edge.towards(100, "Corridor");
        edge.towards(200, "Corner");
        edge.towards(5, "End");

        let mut edge = graph.new_edge("Corner");
        edge.towards(1, "Corridor");
        println!("{:?}", graph);
        let mut traversal = graph.new_traversal("Start", || {
            let cur = std::time::UNIX_EPOCH.elapsed()
                .unwrap().as_nanos();
            let val = (cur as f64 % 100000.0 ) / (100000 as f64);
            println!("{}", val);
            val as f32
        });
        loop {
            let e = traversal.choose();
            println!("{:?}", e);
            if e == None {
                break;
            }
        }
    }
}
