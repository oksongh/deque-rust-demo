use std::{
    cell::RefCell,
    fmt::{self, Debug},
    rc::Rc,
};

#[derive(Clone, Debug)]
struct Node<T: Clone> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}
struct DoublyLinkedList<T: Clone> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}
impl<T: Clone> DoublyLinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    fn pop_back(&mut self) -> T {
        unimplemented!()
    }
    fn pop_front(&mut self) -> T {
        unimplemented!()
    }
    fn push_back(&mut self, elm: T) {
        let new_tail = Rc::new(RefCell::new(Node {
            data: elm,
            next: None,
            prev: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
                new_tail.borrow_mut().prev = Some(old_tail);
            }
            None => {
                // first node
                self.head = Some(Rc::clone(&new_tail));
            }
        }
        self.tail = Some(Rc::clone(&new_tail));
    }
    fn push_front(&mut self, elm: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data: elm,
            next: None,
            prev: None,
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&new_node));
                new_node.borrow_mut().next = Some(old_head);
            }
            None => {
                // first node
                self.tail = Some(Rc::clone(&new_node));
            }
        }
        self.head = Some(Rc::clone(&new_node));
    }
}
impl<T: Clone + Debug> From<&[T]> for DoublyLinkedList<T> {
    fn from(elms: &[T]) -> DoublyLinkedList<T> {
        let mut list = DoublyLinkedList::new();
        elms.iter().for_each(|e| {
            list.push_back(e.clone());
        });
        list
    }
}
impl<T: Clone + Debug, const N: usize> From<&[T; N]> for DoublyLinkedList<T> {
    fn from(elms: &[T; N]) -> DoublyLinkedList<T> {
        let mut list = DoublyLinkedList::new();
        elms.iter().for_each(|e| {
            list.push_back(e.clone());
        });
        list
    }
}
impl<T> fmt::Display for DoublyLinkedList<T>
where
    T: Clone + fmt::Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head.clone();
        write!(f, "(")?;

        while let Some(node) = current {
            // let n = RefCell::borrow(&node);
            let n = node.borrow();
            write!(f, "{}", n.data)?;
            // write!(f, "@{:p}", Rc::as_ptr(&node))?;

            current = n.next.clone();
            if current.is_some() {
                write!(f, "<--->")?;
            }
        }
        write!(f, ")")?;

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_back1() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        println!("{list}");
        assert_eq!(list.to_string(), "(1)")
    }
    #[test]
    fn push_back1_3() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.to_string(), "(1<--->2<--->3)")
    }
    #[test]
    fn push_front1_3() {
        let mut list = DoublyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.to_string(), "(3<--->2<--->1)")
    }
    #[test]
    fn push_back_front() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_front(4);
        list.push_front(5);
        assert_eq!(list.to_string(), "(5<--->4<--->1<--->2<--->3)");
    }
    #[test]
    fn from() {
        let list = DoublyLinkedList::from(&[1, 2, 3, 4, 5][..]);
        assert_eq!(list.to_string(), "(1<--->2<--->3<--->4<--->5)");
    }

    #[test]
    fn from_const_size() {
        let list = DoublyLinkedList::from(&[1, 2, 3, 4, 5]);
        assert_eq!(list.to_string(), "(1<--->2<--->3<--->4<--->5)");
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    list.push_back(1);
    assert_eq!(list.to_string(), "(1)");

    let a = std::cell::Cell::new(10); // immutable object with interior mutability
    let b = a.replace(20);
    dbg!(a.get()); // a.get() = 20
    dbg!(b); // b = 10

    let c = a.clone().into_inner(); // turn Cell<T> into T
    dbg!(c); // c = 20
    dbg!(a); // borrow check - Error

    // let s = std::cell::Cell::new(String::from("aaaa"));
}
