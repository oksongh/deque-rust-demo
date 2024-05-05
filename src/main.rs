use std::{
    borrow::{Borrow, BorrowMut},
    fmt,
    rc::Rc,
};

#[derive(Clone)]
struct Node<T: Clone> {
    data: T,
    next: Option<Rc<Node<T>>>,
    prev: Option<Rc<Node<T>>>,
}
struct DoublyLinkedList<T: Clone> {
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
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
        if self.head.is_none() {
            let node = Rc::new(Node {
                data: elm,
                next: self.tail.clone(),
                prev: self.head.clone(),
            });
            self.head = Some(node.clone());
            self.tail = Some(node.clone());

            return;
        }

        // match &mut self.tail {
        //     Some(tail) => {
        //         let node = Rc::new(Node {
        //             data: elm,
        //             next: None,
        //             // prev: self.tail.clone(),
        //             prev: None,
        //         });

        //         tail.next = Some(node);
        //     }
        //     None => {}
        // }
    }
    fn push_front(&mut self, elm: T) {
        unimplemented!()
    }
}
impl<T> fmt::Display for DoublyLinkedList<T>
where
    T: Clone + fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head.clone();
        write!(f, "(")?;
        while let Some(ref node) = current {
            // let n = node;
            write!(f, "{}", node.data)?;
            current = node.next.clone();
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
}
fn main() {
    let mut list = DoublyLinkedList::new();
    list.push_back(1);
    assert_eq!(list.to_string(), "(1)")
}
