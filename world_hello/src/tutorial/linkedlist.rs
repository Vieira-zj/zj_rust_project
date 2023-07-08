use std::{cell::RefCell, rc::Rc};

// Refer: https://cmooneycollett.github.io/2023/07/01/writing-a-linkedlist-in-rust

// non thread-safe
type Link<T> = Option<Rc<RefCell<Box<Node<T>>>>>;

// Node

/// A node containing a data item and links to previous and next nodes.
struct Node<T: std::fmt::Debug> {
    data: Rc<T>,
    prev: Link<T>,
    next: Link<T>,
}

#[allow(unused)]
impl<T: std::fmt::Debug> Node<T> {
    /// Creates a new Node containing the given data item.
    /// The previous and next node links are set to None.
    fn new(data: T) -> Node<T> {
        Node {
            data: Rc::new(data),
            prev: None,
            next: None,
        }
    }

    /// Creates a new Link containing the given data item.
    fn new_link(data: T) -> Link<T> {
        Some(Rc::new(RefCell::new(Box::new(Node::new(data)))))
    }

    fn get_data(&self) -> Rc<T> {
        self.data.clone()
    }

    /// Updates the previous node.
    fn set_prev(&mut self, node: &Link<T>) {
        self.prev = node.clone();
    }
    /// Updates the next node.
    fn set_next(&mut self, node: &Link<T>) {
        self.next = node.clone();
    }

    /// Gets the previous link from the Node via cloning.
    fn get_prev(&self) -> Link<T> {
        self.prev.clone()
    }
    /// Gets the next link from the Node via cloning.
    fn get_next(&self) -> Link<T> {
        self.next.clone()
    }
}

// LinkedList

/// An implementation of a doubly linked-list. Not thread-safe. Note that the
/// data items contained within nodes cannot be changed after they have been
/// added to the linked-list.
struct LinkedList<T: std::fmt::Debug> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

#[allow(unused)]
impl<T: std::fmt::Debug> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /// Pushes the data item to the end of the LinkedList.
    pub fn push(&mut self, data: T) {
        let new_node: Link<T> = Node::new_link(data);
        self.len += 1;
        if self.head.is_none() && self.tail.is_none() {
            self.head = new_node.clone();
            self.tail = new_node;
            return;
        }

        self.tail.as_ref().unwrap().borrow_mut().set_next(&new_node);
        new_node.as_ref().unwrap().borrow_mut().set_prev(&self.tail);
        self.tail = new_node;
    }

    /// Pushes the data item to the front of the LinkedList.
    pub fn push_front(&mut self, data: T) {
        let new_node: Link<T> = Node::new_link(data);
        self.len += 1;
        if self.head.is_none() && self.tail.is_none() {
            self.head = new_node.clone();
            self.tail = new_node;
            return;
        }

        self.head.as_ref().unwrap().borrow_mut().set_prev(&new_node);
        new_node.as_ref().unwrap().borrow_mut().set_next(&self.head);
        self.head = new_node;
    }

    /// Removes the last node from the LinkedList. Returns Some containing the value
    /// from the removed node, otherwise None.
    pub fn pop(&mut self) -> Option<Rc<T>> {
        if self.head.is_none() && self.tail.is_none() {
            return None;
        }

        let old_tail = self.tail.clone();
        self.tail = old_tail.as_ref().unwrap().borrow().get_prev();
        self.tail.as_ref().unwrap().borrow_mut().set_next(&None);

        self.len -= 1;
        let old_data = old_tail.as_ref().unwrap().borrow().get_data();
        Some(old_data)
    }

    /// Removes the first node from the LinkedList. Returns Some containing the
    /// value from the removed node, otherwise None.
    pub fn pop_front(&mut self) -> Option<Rc<T>> {
        if self.head.is_none() && self.tail.is_none() {
            return None;
        }

        let old_head = self.head.clone();
        self.head = old_head.as_ref().unwrap().borrow().get_next();
        self.head.as_ref().unwrap().borrow_mut().set_prev(&None);

        self.len -= 1;
        let old_data = old_head.as_ref().unwrap().borrow().get_data();
        Some(old_data)
    }

    /// Returns the number of items contained in the LinkedList.
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn print(&self) {
        if self.head.is_none() {
            println!("values: [null]")
        }

        print!("values: [");
        let mut cur = self.head.clone();
        while let Some(node) = cur {
            print!("{:?},", node.borrow().get_data());
            cur = node.borrow().get_next();
        }
        print!("]\n");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_push_back_length() {
        let mut new_list = LinkedList::<i32>::new();
        for i in (0..10).into_iter() {
            new_list.push(i);
        }

        assert!(!new_list.is_empty());
        assert_eq!(new_list.len(), 10);
        new_list.print();
    }

    #[test]
    fn test_push_front_length() {
        let mut new_list = LinkedList::<i32>::new();
        for i in (0..10).into_iter() {
            new_list.push_front(i);
        }

        assert!(!new_list.is_empty());
        assert_eq!(new_list.len(), 10);
        new_list.print();
    }
}
