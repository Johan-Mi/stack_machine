use super::value::Value;
use std::fmt::{self, Display};
use std::rc::Rc;

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
    pub fn iter(&self) -> ListIter {
        ListIter::new(&self.head)
    }
}

impl Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}", link_to_string(&self.head))
    }
}

fn link_to_string(this: &Link) -> String {
    match this {
        Some(node) => {
            format!("{} {}", node.value, link_to_string(&node.next))
        }
        None => String::from("]"),
    }
}

#[macro_export]
macro_rules! make_list {
    ($($all:expr),*) => {
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

pub struct ListIter<'a> {
    link: &'a Link,
}

impl<'a> ListIter<'a> {
    pub fn new(link: &'a Link) -> Self {
        Self { link }
    }
}

impl<'a> Iterator for ListIter<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self.link {
            Some(node) => {
                self.link = &node.next;
                Some(&node.value)
            }
            None => None,
        }
    }
}
