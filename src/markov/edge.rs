use std::hash::Hash;

use crate::markov::choice::Choice;
use crate::markov::link::Link;
use std::collections::HashMap;


trait ToBox<T> {
    fn into_box(self) -> Box<T>;
}

impl<T> ToBox<T> for T where T: Sized {
    fn into_box(self) -> Box<T> { Box::new(self) }
}

#[derive(Debug)]
pub struct Edge<'a, T> where T: Clone + Eq + Hash {
    pub value_to_link: &'a mut HashMap<Box<T>, Link<T>>,
    pub value: Box<T>,
}


impl<'a, T> Edge<'a, T> where T: Clone + Eq + Hash {
    pub fn towards(&mut self, weight: i32, value: T) -> &mut Edge<'a, T> {
        let value = value.into_box();
        let choice = Choice {
            weight,
            value: value.clone(),
        }.into_box();

        let link = self.value_to_link
            .entry(self.value.clone())
            .or_insert(Link::default());
        link.list.push(choice);
        link.list.sort_by_key(|e| {e.weight});
        link.total_weight += weight;
        self
    }
}


#[cfg(test)]
mod tests {
    use std::collections::{HashMap};

    use crate::markov::choice::Choice;
    use crate::markov::graph::{Graph};
    use crate::markov::link::Link;
    use crate::markov::edge::ToBox;

    #[test]
    fn hash_with_boxed_key_modifies_same_value() {
        let mut hash = HashMap::new();
        let a = Box::new(2);
        let b = Box::new(2);
        hash.insert(a, "a");
        hash.insert(b, "b");

        assert_eq!(hash.entry(Box::new(2)).or_insert("none"), &"b");
    }

    #[test]
    fn edge_add() {
        let mut link = Graph::default();
        let mut edge = link.new_edge("Start");
        edge.towards(2, "Corridor");
        edge.towards(1, "End");

        let result = link.value_to_link
            .get(&"Start")
            .to_owned()
            .cloned();

        assert_eq!(result, Some(Link {
            total_weight: 3,
            list: vec![
                Box::from(Choice {
                    value: "Corridor".into_box(),
                    weight: 2,
                }),
                Box::from(Choice {
                    value: "End".into_box(),
                    weight: 1,
                }),
            ],
        }));
    }
}
