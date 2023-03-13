// first.rs

use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

pub struct ListIterator<'a> {
    current: &'a Link,
}

impl List {
    pub fn iter(&self) -> ListIterator {
        ListIterator {
            current: &self.head,
        }
    }

    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Iterator for ListIterator<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Link::Empty => None,
            Link::More(node) => {
                self.current = &node.next;
                Some(&node.elem)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
