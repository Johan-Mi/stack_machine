use super::value::Value;
use std::fmt::{self, Display};
use std::rc::Rc;

#[derive(Clone)]
pub enum List {
    Cons { value: Value, next: Rc<List> },
    None,
}

impl List {
    pub fn iter(&self) -> ListIter {
        ListIter::new(self)
    }
}

impl Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            List::Cons { value, next } => {
                write!(f, "[{}{}]", value, list_to_string(next))
            }
            List::None => write!(f, "[]"),
        }
    }
}

fn list_to_string(this: &List) -> String {
    match this {
        List::Cons { value, next } => {
            format!(" {}{}", value, list_to_string(next))
        }
        List::None => String::from(""),
    }
}

#[macro_export]
macro_rules! make_list {
    () => {
        $crate::list::List::None
    };

    ($first:expr $(, $rest:expr)*) => {
        $crate::list::List::Cons {
            value: $first,
            next: ::std::rc::Rc::new(make_list!($($rest),*)),
        }
    };
}

pub struct ListIter<'a> {
    node: &'a List,
}

impl<'a> ListIter<'a> {
    pub fn new(list: &'a List) -> Self {
        Self { node: list }
    }
}

impl<'a> Iterator for ListIter<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node {
            List::Cons { value, next } => {
                self.node = next;
                Some(value)
            }
            List::None => None,
        }
    }
}
