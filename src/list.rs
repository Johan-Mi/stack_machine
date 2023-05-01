use crate::value::Value;
use derive_more::Constructor;
use itertools::Itertools;
use std::{
    fmt::{self, Display},
    rc::Rc,
};

#[derive(Clone)]
pub struct List {
    pub head: Link,
}

type Link = Option<Rc<Node>>;

pub struct Node {
    pub value: Value,
    pub next: Link,
}

impl List {
    pub fn iter(&self) -> Iter {
        Iter::new(self.head.as_ref().map(Rc::as_ref))
    }
}

impl Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.iter().format(" "))
    }
}

#[macro_export]
macro_rules! make_list {
    ($($all:expr),* $(,)?) => {
        $crate::list::List {
            head: make_list!(@a $($all),*)
        }
    };

    (@a) => {
        None
    };

    (@a $first:expr $(, $rest:expr)*) => {
        Some(::std::rc::Rc::new($crate::list::Node {
            value: $first, next: make_list!(@a $($rest),*)
        }))
    };
}

#[derive(Constructor)]
pub struct Iter<'a> {
    node: Option<&'a Node>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;
        self.node = node.next.as_ref().map(Rc::as_ref);
        Some(&node.value)
    }
}
